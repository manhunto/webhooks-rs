use std::fmt::{Display, Formatter};

use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::configuration::domain::{ApplicationId, Endpoint, EndpointId, Topic};

#[derive(Debug, Clone)]
pub struct Payload {
    body: String,
}

impl From<Value> for Payload {
    fn from(value: Value) -> Self {
        Self {
            body: value.to_string(),
        }
    }
}

impl Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let body: Value = serde_json::from_str(self.body.to_string().as_str()).unwrap();

        serializer.serialize_some(&body)
    }
}

impl Display for Payload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

#[derive(Debug, Clone, derive::Ksuid, Eq, PartialEq)]
#[prefix = "msg"]
pub struct MessageId {
    id: String,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: MessageId,
    pub app_id: ApplicationId,
    pub payload: Payload,
    pub topic: Topic,
}

impl Message {
    pub fn new(app_id: ApplicationId, payload: Payload, topic: Topic) -> Self {
        Self {
            id: MessageId::new(),
            app_id,
            payload,
            topic,
        }
    }
}

#[derive(Debug, Clone, derive::Ksuid, Eq, PartialEq)]
#[prefix = "rmsg"]
pub struct RoutedMessageId {
    id: String,
}

#[derive(Debug, Clone)]
pub struct RoutedMessage {
    pub id: RoutedMessageId,
    pub msg_id: MessageId,
    pub endpoint_id: EndpointId,
}

impl From<(Message, Endpoint)> for RoutedMessage {
    fn from(value: (Message, Endpoint)) -> Self {
        let (msg, endpoint) = value;

        Self::new(msg.id, endpoint.id)
    }
}

impl RoutedMessage {
    fn new(msg_id: MessageId, endpoint_id: EndpointId) -> Self {
        Self {
            id: RoutedMessageId::new(),
            msg_id,
            endpoint_id,
        }
    }
}
