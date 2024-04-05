use std::time::Duration;

use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable};
use log::{debug, info};

use server::amqp::{establish_connection_with_rabbit, Publisher, SENT_MESSAGE_QUEUE, Serializer};
use server::cmd::AsyncMessage;
use server::logs::init_log;
use server::retry::{
    ExponentialRetryPolicy, RandomizeDecoratedRetryPolicy, RetryPolicy,
};

#[tokio::main]
async fn main() {
    init_log();

    let channel = establish_connection_with_rabbit().await;
    let retry_policy = RandomizeDecoratedRetryPolicy::new(
        Box::new(ExponentialRetryPolicy::new(5, 2, Duration::from_secs(2))),
        0.5,
    );
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

        if response.is_err() && retry_policy.is_retryable(cmd.attempt) {
            let cmd_to_retry = cmd.with_increased_attempt();
            let duration = retry_policy.get_waiting_time(cmd.attempt);

            publisher
                .publish_delayed(AsyncMessage::SentMessage(cmd_to_retry.clone()), duration)
                .await;

            debug!(
                "Message queued again. Attempt: {}. Delay: {:?}",
                cmd_to_retry.attempt,
                duration
            );
        }

        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
}
