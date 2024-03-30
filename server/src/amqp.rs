use std::collections::BTreeMap;

use lapin::{Channel, Connection, ConnectionProperties, ExchangeKind};
use lapin::options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions};
use lapin::types::{AMQPType, AMQPValue, FieldTable, ShortString};
use log::info;
use serde_json::Value;

pub const SENT_MESSAGE_QUEUE: &str = "sent-message";

pub async fn establish_connection_with_rabbit() -> Channel {
    let addr = "amqp://guest:guest@localhost:5672";
    let conn = Connection::connect(addr, ConnectionProperties::default())
        .await
        .unwrap();

    info!("connected established with rabbitmq");

    let channel = conn.create_channel().await.unwrap();

    let args = FieldTable::from(BTreeMap::from(
        [(
            ShortString::from("x-delayed-type"),
            AMQPValue::try_from(&Value::String(String::from("direct")), AMQPType::LongString)
                .unwrap(),
        ); 1],
    ));

    channel
        .exchange_declare(
            "sent-message-exchange",
            ExchangeKind::Custom(String::from("x-delayed-message")),
            ExchangeDeclareOptions::default(),
            args,
        )
        .await
        .unwrap();

    let queue = channel
        .queue_declare(
            SENT_MESSAGE_QUEUE,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    channel
        .queue_bind(
            queue.name().as_str(),
            "sent-message-exchange",
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    info!("queue declared {:?}", queue);

    channel
}
