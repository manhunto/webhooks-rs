use std::time::Duration;

use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable};
use log::{debug, info};

use server::amqp::{establish_connection_with_rabbit, Publisher, Serializer, SENT_MESSAGE_QUEUE};
use server::circuit_breaker::{CircuitBreaker, State};
use server::cmd::AsyncMessage;
use server::logs::init_log;
use server::retry::RetryPolicyBuilder;

#[tokio::main]
async fn main() {
    init_log();

    let retry_policy = RetryPolicyBuilder::new()
        .max_retries(5)
        .exponential(2, Duration::from_secs(2))
        .randomize(0.5)
        .build()
        .unwrap();

    let mut circuit_breaker = CircuitBreaker::new();

    let channel = establish_connection_with_rabbit().await;
    let mut consumer = channel
        .basic_consume(
            SENT_MESSAGE_QUEUE,
            "dispatcher",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    let publisher = Publisher::new(channel);

    info!("consumer is ready");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        let async_msg: AsyncMessage = Serializer::deserialize(&delivery.data);

        let AsyncMessage::SentMessage(cmd) = async_msg;

        info!("message consumed: {:?}", cmd);

        // todo allow to revive endpoint, check endpoint status and reset circuit breaker
        let key = cmd.endpoint_id.to_string();
        if circuit_breaker.is_call_permitted(key.clone()) {
            debug!(
                "Endpoint {} is closed. Message {} skipped.",
                key, cmd.msg_id
            );

            // todo do something with message? add to some "not delivered" bucket
        } else {
            let response = reqwest::Client::new()
                .post(&cmd.url)
                .json(&cmd.payload.as_str())
                .send()
                .await;

            let dbg_msg = match &response {
                Ok(res) => format!("Success! {}", res.status()),
                Err(res) => {
                    let status: String = res.status().map_or(String::from("-"), |s| s.to_string());

                    format!("Error response! Status: {}, Error: {}", status, res)
                }
            };

            debug!("{}", dbg_msg);

            match circuit_breaker.call(key, response) {
                State::Close => {
                    // todo mark endpoint as disabled
                }
                State::Open => {
                    if retry_policy.is_retryable(cmd.attempt) {
                        let cmd_to_retry = cmd.with_increased_attempt();
                        let duration = retry_policy.get_waiting_time(cmd.attempt);

                        publisher
                            .publish_delayed(
                                AsyncMessage::SentMessage(cmd_to_retry.clone()),
                                duration,
                            )
                            .await;

                        debug!(
                            "Message queued again. Attempt: {}. Delay: {:?}",
                            cmd_to_retry.attempt, duration
                        );
                    }
                }
            }
        }

        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
}
