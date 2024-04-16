use std::fmt::{Display, Formatter};

use crate::configuration::domain::{ApplicationId, Topic};

#[derive(Debug, Clone)]
pub struct Payload {
    body: String,
}

impl From<String> for Payload {
    fn from(value: String) -> Self {
        Self { body: value }
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
