use std::collections::BTreeMap;
use std::time::Duration;

use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind};
use lapin::options::{
    BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
};
use lapin::publisher_confirm::Confirmation;
use lapin::types::{AMQPType, AMQPValue, FieldTable, ShortString};
use log::info;
use serde_json::Value;

use crate::cmd::AsyncMessage;

pub const SENT_MESSAGE_QUEUE: &str = "sent-message";
const SENT_MESSAGE_EXCHANGE: &str = "sent-message-exchange";

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
            SENT_MESSAGE_EXCHANGE,
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
            SENT_MESSAGE_EXCHANGE,
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    info!("queue declared {:?}", queue);

    channel
}

pub struct Dispatcher {
    channel: Channel,
}

impl Dispatcher {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    pub async fn publish(&self, message: AsyncMessage) {
        self.do_publish(message, BasicProperties::default()).await
    }

    pub async fn publish_delayed(&self, message: AsyncMessage, delay: Duration) {
        let btree: BTreeMap<_, _> = [(
            ShortString::from("x-delay"),
            AMQPValue::LongLongInt(delay.as_millis() as i64),
        )]
            .into();
        let headers = FieldTable::from(btree);
        let properties = BasicProperties::default().with_headers(headers);

        self.do_publish(message, properties).await
    }

    fn resolve_exchange(&self, message: &AsyncMessage) -> &str {
        match message {
            AsyncMessage::SentMessage(_) => SENT_MESSAGE_EXCHANGE,
        }
    }

    async fn do_publish(&self, message: AsyncMessage, properties: BasicProperties) {
        let confirm = self
            .channel
            .basic_publish(
                self.resolve_exchange(&message),
                "",
                BasicPublishOptions::default(),
                serde_json::to_string(&message).unwrap().as_bytes(),
                properties,
            )
            .await
            .unwrap()
            .await
            .unwrap();

        assert_eq!(confirm, Confirmation::NotRequested);
    }
}
