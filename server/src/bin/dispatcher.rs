use std::collections::BTreeMap;

use futures_lite::stream::StreamExt;
use lapin::publisher_confirm::Confirmation;
use lapin::types::{AMQPValue, ShortString};
use lapin::{options::*, types::FieldTable, BasicProperties};
use log::{debug, info};

use server::amqp::{establish_connection_with_rabbit, SENT_MESSAGE_QUEUE};
use server::cmd::SentMessage;
use server::logs::init_log;

#[tokio::main]
async fn main() {
    init_log();

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

    info!("consumer is ready");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        let msg = String::from_utf8_lossy(&delivery.data);
        let cmd: SentMessage = serde_json::from_str(&msg).unwrap();

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

        if response.is_err() && cmd.attempt < 5 {
            let btree: BTreeMap<_, _> =
                [(ShortString::from("x-delay"), AMQPValue::from(5000))].into();
            let headers = FieldTable::from(btree);
            let properties = BasicProperties::default().with_headers(headers);

            let cmd_to_retry = cmd.with_increased_attempt();

            let confirm = channel
                .basic_publish(
                    "sent-message-exchange",
                    "",
                    BasicPublishOptions::default(),
                    serde_json::to_string(&cmd_to_retry).unwrap().as_bytes(),
                    properties,
                )
                .await
                .unwrap()
                .await
                .unwrap();

            assert_eq!(confirm, Confirmation::NotRequested);

            debug!(
                "Message {} queued again. Attempt: {}",
                cmd_to_retry.msg_id, cmd_to_retry.attempt
            );
        }

        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
}
