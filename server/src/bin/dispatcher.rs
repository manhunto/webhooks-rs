use std::time::Duration;

use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable};
use log::{debug, info};

use server::amqp::{Dispatcher, establish_connection_with_rabbit, SENT_MESSAGE_QUEUE};
use server::cmd::{AsyncMessage, SentMessage};
use server::logs::init_log;
use server::retry::{ExponentialRetryPolicy, Retryable, RetryPolicy};

#[tokio::main]
async fn main() {
    init_log();

    let channel = establish_connection_with_rabbit().await;
    let retry_policy = ExponentialRetryPolicy::new(5, 2, Duration::from_secs(2));
    let mut consumer = channel
        .basic_consume(
            SENT_MESSAGE_QUEUE,
            "dispatcher",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    let dispatcher = Dispatcher::new(channel);

    info!("consumer is ready");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        let msg = String::from_utf8_lossy(&delivery.data);
        let async_msg: AsyncMessage = serde_json::from_str(&msg).unwrap();

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
            retry(cmd, &retry_policy, &dispatcher).await;
        }

        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
}

async fn retry(
    sent_message: SentMessage,
    retry_policy: &impl RetryPolicy,
    dispatcher: &Dispatcher,
) {
    if !retry_policy.is_retryable(&sent_message) {
        return;
    }

    let waiting_time = retry_policy.get_waiting_time(&sent_message);
    let cmd_to_retry = sent_message.with_increased_attempt();

    dispatcher
        .publish_delayed(
            AsyncMessage::SentMessage(cmd_to_retry.clone()),
            waiting_time,
        )
        .await;

    debug!(
        "Message queued again. Attempt: {}. Delay: {:?}",
        cmd_to_retry.attempt(),
        waiting_time
    );
}
