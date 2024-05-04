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
    attempts: AttemptCollection,
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
            attempts: AttemptCollection::new(),
        }
    }

    pub fn record_attempt(&mut self, result: SentResult, processing_time: Duration) {
        self.attempts.push(result.status, processing_time)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Attempt {
    #[allow(dead_code)]
    id: u16,
    status: Status,
    #[allow(dead_code)]
    processing_time: Duration,
}

impl Attempt {
    fn new(id: u16, status: Status, processing_time: Duration) -> Result<Self, String> {
        if id < 1 {
            return Err(format!("Id should be greater than 0. Was {}", id));
        }

        Ok(Self {
            id,
            status,
            processing_time,
        })
    }

    fn is_delivered(&self) -> bool {
        match self.status {
            Status::Numeric(status) => (200..=299).contains(&status),
            Status::Unknown(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
struct AttemptCollection {
    attempts: Vec<Attempt>,
}

impl AttemptCollection {
    fn new() -> Self {
        Self {
            attempts: Vec::new(),
        }
    }

    // todo add clock here or to logs?
    fn push(&mut self, status: Status, processing_time: Duration) {
        let attempt =
            Attempt::new(self.attempts.len() as u16 + 1, status, processing_time).unwrap();

        if self.attempts.iter().any(|a| a.is_delivered()) {
            panic!("Could not push to the attempt collection when was delivered");
        }

        self.attempts.push(attempt)
    }

    #[cfg(test)]
    fn all(&self) -> Vec<Attempt> {
        let mut vec = self.attempts.clone();
        vec.sort_unstable_by(|a, b| a.id.cmp(&b.id));

        vec
    }
}

#[cfg(test)]
mod attempt_test {
    use std::time::Duration;

    use test_case::test_case;

    use crate::events::domain::Attempt;
    use crate::sender::Status;

    #[test]
    #[should_panic]
    fn attempt_id_should_be_greater_than_0() {
        Attempt::new(
            0,
            Status::Unknown("Test".to_string()),
            Duration::from_millis(10),
        )
        .unwrap();
    }

    #[test_case(Status::Numeric(200), true)]
    #[test_case(Status::Numeric(201), true)]
    #[test_case(Status::Numeric(299), true)]
    #[test_case(Status::Numeric(300), false)]
    #[test_case(Status::Numeric(400), false)]
    #[test_case(Status::Numeric(502), false)]
    #[test_case(Status::Unknown("test".to_string()), false)]
    fn attempt_is_delivered(status: Status, expected: bool) {
        let sut = Attempt::new(1, status, Duration::from_millis(1)).unwrap();

        assert_eq!(expected, sut.is_delivered());
    }
}

#[cfg(test)]
mod attempt_collection_test {
    use std::time::Duration;

    use crate::events::domain::AttemptCollection;
    use crate::sender::Status::Numeric;

    #[test]
    fn get_attempts_from_collection() {
        let mut sut = AttemptCollection::new();

        sut.push(Numeric(504), Duration::from_millis(10));
        sut.push(Numeric(502), Duration::from_millis(10));
        sut.push(Numeric(500), Duration::from_millis(10));
        sut.push(Numeric(400), Duration::from_millis(10));
        sut.push(Numeric(200), Duration::from_millis(10));

        let mut vec = sut.all().into_iter();

        assert_eq!(Numeric(504), vec.next().unwrap().status);
        assert_eq!(Numeric(502), vec.next().unwrap().status);
        assert_eq!(Numeric(500), vec.next().unwrap().status);
        assert_eq!(Numeric(400), vec.next().unwrap().status);
        assert_eq!(Numeric(200), vec.next().unwrap().status);
        assert_eq!(None, vec.next());
    }

    #[test]
    #[should_panic]
    fn cannot_push_attempt_when_collection_is_delivered() {
        let mut sut = AttemptCollection::new();

        sut.push(Numeric(200), Duration::from_millis(10));
        sut.push(Numeric(200), Duration::from_millis(10));
    }
}
