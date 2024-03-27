use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable};
use log::info;
use server::amqp::establish_connection_with_rabbit;
use server::cmd::SentMessage;
use server::logs::init_log;

#[tokio::main]
async fn main() {
    init_log();

    let channel = establish_connection_with_rabbit().await;

    let mut consumer = channel
        .basic_consume(
            "sent_message",
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

        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
}
