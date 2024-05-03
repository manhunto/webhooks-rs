use std::fmt::{Display, Formatter};
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::configuration::domain::{Endpoint, Topic};
use crate::sender::{SentResult, Status};
use crate::time::Clock;
use crate::types::{ApplicationId, EndpointId, MessageId, RoutedMessageId};

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

#[derive(Debug, Clone)]
pub struct Message {
    pub id: MessageId,
    pub app_id: ApplicationId,
    pub payload: Payload,
    pub topic: Topic,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(app_id: ApplicationId, payload: Payload, topic: Topic, clock: &Clock) -> Self {
        Self {
            id: MessageId::new(),
            app_id,
            payload,
            topic,
            created_at: clock.now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoutedMessage {
    pub id: RoutedMessageId,
    pub msg_id: MessageId,
    pub endpoint_id: EndpointId,
    attempts: Vec<Attempt>,
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
            attempts: Vec::new(),
        }
    }

    pub fn record_attempt(&mut self, result: SentResult, processing_time: Duration) {
        self.attempts
            .push(self.create_attempt(result.status, processing_time))
    }

    fn create_attempt(&self, status: Status, processing_time: Duration) -> Attempt {
        Attempt {
            id: self.attempts.len() as u16 + 1,
            status,
            processing_time,
        }
    }
}

#[derive(Debug, Clone)]
struct Attempt {
    #[allow(dead_code)]
    id: u16,
    #[allow(dead_code)]
    status: Status,
    #[allow(dead_code)]
    processing_time: Duration,
}
