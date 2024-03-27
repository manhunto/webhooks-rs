use lapin::options::QueueDeclareOptions;
use lapin::types::FieldTable;
use lapin::{Channel, Connection, ConnectionProperties};
use log::info;

pub async fn establish_connection_with_rabbit() -> Channel {
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

    channel
}
