use std::time::Duration;

use futures_lite::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use lapin::Channel;
use log::{debug, info};

use crate::amqp::{Publisher, Serializer, SENT_MESSAGE_QUEUE};
use crate::circuit_breaker::{CircuitBreaker, Error};
use crate::cmd::AsyncMessage;
use crate::retry::RetryPolicyBuilder;

pub async fn consume(channel: Channel, consumer_tag: &str) {
    let retry_policy = RetryPolicyBuilder::new()
        .max_retries(5)
        .exponential(2, Duration::from_secs(2))
        .randomize(0.5)
        .build()
        .unwrap();

    let mut circuit_breaker = CircuitBreaker::new();

    let mut consumer = channel
        .basic_consume(
            SENT_MESSAGE_QUEUE,
            consumer_tag,
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
        let func = || {
            reqwest::blocking::Client::new()
                .post(&cmd.url)
                .json(&cmd.payload.as_str())
                .send()
        };

        let log_error_response = |res: reqwest::Error| {
            let status: String = res.status().map_or(String::from("-"), |s| s.to_string());

            debug!("Error response! Status: {}, Error: {}", status, res);
        };

        match circuit_breaker.call(key.clone(), func) {
            Ok(res) => {
                debug!("Success! {}", res.status())
            }
            Err(err) => match err {
                Error::Closed(res) => {
                    log_error_response(res);

                    // todo mark endpoint as disabled
                }
                Error::Open(res) => {
                    log_error_response(res);

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
                Error::Rejected => {
                    debug!(
                        "Endpoint {} is closed. Message {} rejected.",
                        key, cmd.msg_id
                    );

                    // todo do something with message? add to some "not delivered" bucket?
                }
            },
        }

        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
}
