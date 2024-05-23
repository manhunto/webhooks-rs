use std::collections::BTreeMap;
use std::time::Duration;

use lapin::options::{
    BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
};
use lapin::publisher_confirm::Confirmation;
use lapin::types::{AMQPType, AMQPValue, FieldTable, ShortString};
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind};
use log::info;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::cmd::AsyncMessage;
use crate::config::AMQPConfig;

pub async fn establish_connection_with_rabbit(amqp_config: AMQPConfig) -> Channel {
    let addr = amqp_config.connection_string();
    let conn = Connection::connect(&addr, ConnectionProperties::default())
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
            &amqp_config.sent_message_exchange_name(),
            ExchangeKind::Custom(String::from("x-delayed-message")),
            ExchangeDeclareOptions::default(),
            args,
        )
        .await
        .unwrap();

    let queue = channel
        .queue_declare(
            &amqp_config.sent_message_queue_name(),
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    channel
        .queue_bind(
            queue.name().as_str(),
            &amqp_config.sent_message_exchange_name(),
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    info!("queue declared {:?}", queue);

    channel
}

pub struct Publisher {
    channel: Channel,
    amqp_config: AMQPConfig,
}

impl Publisher {
    pub fn new(channel: Channel, amqp_config: AMQPConfig) -> Self {
        Self {
            channel,
            amqp_config,
        }
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

    fn resolve_exchange(&self, message: &AsyncMessage) -> String {
        match message {
            AsyncMessage::SentMessage(_) => self.amqp_config.sent_message_exchange_name().clone(),
        }
    }

    async fn do_publish(&self, message: AsyncMessage, properties: BasicProperties) {
        let confirm = self
            .channel
            .basic_publish(
                self.resolve_exchange(&message).as_str(),
                "",
                BasicPublishOptions::default(),
                &Serializer::serialize(message),
                properties,
            )
            .await
            .unwrap()
            .await
            .unwrap();

        assert_eq!(confirm, Confirmation::NotRequested);
    }
}

pub struct Serializer {}

impl Serializer {
    pub fn deserialize<T>(binary: &[u8]) -> T
    where
        T: DeserializeOwned,
    {
        let msg = String::from_utf8_lossy(binary);

        serde_json::from_str(&msg).unwrap()
    }

    pub fn serialize<T>(value: T) -> Vec<u8>
    // is possible to return &[u8] ?
    where
        T: Serialize,
    {
        let string = serde_json::to_string(&value);

        string.unwrap().as_bytes().to_vec()
    }
}
