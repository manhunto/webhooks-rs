use std::collections::HashMap;
use std::time::Duration;

use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable};
use log::{debug, info};

use server::amqp::{establish_connection_with_rabbit, Publisher, Serializer, SENT_MESSAGE_QUEUE};
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

    let mut fail_map: HashMap<String, u32> = HashMap::new();

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

        if response.is_err() {
            if retry_policy.is_retryable(cmd.attempt) {
                let cmd_to_retry = cmd.with_increased_attempt();
                let duration = retry_policy.get_waiting_time(cmd.attempt);

                publisher
                    .publish_delayed(AsyncMessage::SentMessage(cmd_to_retry.clone()), duration)
                    .await;

                debug!(
                    "Message queued again. Attempt: {}. Delay: {:?}",
                    cmd_to_retry.attempt, duration
                );
            }

            let key = cmd.endpoint_id.to_string();

            *fail_map.entry(key.clone()).or_insert(0) += 1;

            let fail_count = fail_map.get(&key).unwrap();

            debug!("Endpoint {} current fail count: {}", key, fail_count);

            if fail_count.ge(&5) {
                // todo mark endpoint as disabled
                debug!("Endpoint {} reached a limit and is disabled", key);
            }
        }

        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
}
