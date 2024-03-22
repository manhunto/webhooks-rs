use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use log::info;
use server::logs::init_log;

#[tokio::main]
async fn main() {
    init_log();

    let addr = "amqp://guest:guest@localhost:5672";

    let conn = Connection::connect(addr, ConnectionProperties::default())
        .await
        .unwrap();

    info!("connected established with rabbitmq");

    let channel = conn.create_channel().await.unwrap();

    let queue = channel
        .queue_declare(
            "sent_message",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    info!("queue declared {:?}", queue);

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

        info!("message consumed: {:?}", msg);

        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
}
