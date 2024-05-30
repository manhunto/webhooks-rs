use std::fmt::{Display, Formatter};
use std::time::Duration;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Serialize, Serializer};
use serde_json::Value;
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

use crate::configuration::domain::{Endpoint, Topic};
use crate::sender::{SentResult, Status};
use crate::time::Clock;
use crate::types::{ApplicationId, AttemptId, EndpointId, EventId, MessageId};

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
pub struct Event {
    pub id: EventId,
    pub app_id: ApplicationId,
    pub payload: Payload,
    pub topic: Topic,
    pub created_at: DateTime<Utc>,
}

impl Event {
    #[must_use]
    pub fn new(app_id: ApplicationId, payload: Payload, topic: Topic, clock: &Clock) -> Self {
        Self {
            id: EventId::new(),
            app_id,
            payload,
            topic,
            created_at: clock.now(),
        }
    }

    #[must_use]
    pub fn calculate_processing_time(&self, clock: &Clock) -> Duration {
        let now = clock.now();
        if now < self.created_at {
            unreachable!(
                "Unable to calculate processing time because created_at_date is after now date"
            );
        }

        let processing_time = now - self.created_at;

        processing_time
            .to_std()
            .unwrap_or_else(|_| Duration::from_secs(i64::MAX as u64)) // fixme: is max correct?
    }
}

impl FromRow<'_, PgRow> for Event {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let created_at: NaiveDateTime = row.try_get("created_at")?;
        let topic: String = row.try_get("topic")?;
        let payload: Value = row.try_get("payload")?;

        Ok(Event {
            id: row.try_get("id")?,
            app_id: row.try_get("app_id")?,
            created_at: created_at.and_utc(),
            topic: Topic::try_from(topic).unwrap(),
            payload: Payload::from(payload),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: MessageId,
    pub event_id: EventId,
    pub endpoint_id: EndpointId,
    pub attempts: AttemptCollection,
}

impl From<(Event, Endpoint)> for Message {
    fn from(value: (Event, Endpoint)) -> Self {
        let (event, endpoint) = value;

        Self::new(event.id, endpoint.id)
    }
}

impl Message {
    fn new(event_id: EventId, endpoint_id: EndpointId) -> Self {
        let id = MessageId::new();

        Self {
            id,
            event_id,
            endpoint_id,
            attempts: AttemptCollection::new(id),
        }
    }

    pub fn record_attempt(&mut self, result: SentResult, processing_time: Duration) -> AttemptLog {
        let id = self.attempts.push(result.status);

        AttemptLog::new(id, processing_time, result.response_time, result.body)
    }

    #[must_use]
    pub fn attempts(&self) -> Vec<Attempt> {
        self.attempts.all()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attempt {
    id: AttemptId,
    status: Status,
}

impl Attempt {
    #[must_use]
    pub fn attempt_id(&self) -> u16 {
        self.id.attempt_no()
    }

    #[must_use]
    pub fn message_id(&self) -> MessageId {
        self.id.message_id()
    }

    #[must_use]
    pub fn status(&self) -> Status {
        self.status.clone()
    }

    fn new(id: AttemptId, status: Status) -> Self {
        Self { id, status }
    }

    fn is_delivered(&self) -> bool {
        match self.status {
            Status::Numeric(status) => (200..=299).contains(&status),
            Status::Unknown(_) => false,
        }
    }
}

impl FromRow<'_, PgRow> for Attempt {
    fn from_row(row: &'_ PgRow) -> Result<Self, Error> {
        let message_id: MessageId = row.try_get("message_id")?;
        let attempt_no: i16 = row.try_get("attempt")?;
        let id = AttemptId::new(message_id, attempt_no as u16).unwrap();

        let status: Status = Status::from_row(row)?;

        Ok(Self { id, status })
    }
}

#[derive(Debug, Clone)]
pub struct AttemptCollection {
    message_id: MessageId,
    attempts: Vec<Attempt>,
}

impl AttemptCollection {
    fn new(message_id: MessageId) -> Self {
        Self {
            message_id,
            attempts: Vec::new(),
        }
    }

    // todo add clock here or to logs?
    // fixme: improve returning id
    fn push(&mut self, status: Status) -> AttemptId {
        let attempt = Attempt::new(self.next_id(), status);

        if self.attempts.iter().any(Attempt::is_delivered) {
            panic!("Could not push to the attempt collection when was delivered");
        }

        let id = attempt.id;
        self.attempts.push(attempt);

        id
    }

    fn next_id(&self) -> AttemptId {
        AttemptId::new(self.message_id, self.attempts.len() as u16 + 1).unwrap()
    }

    fn all(&self) -> Vec<Attempt> {
        let mut vec = self.attempts.clone();
        vec.sort_unstable_by(|a, b| a.id.attempt_no().cmp(&b.id.attempt_no()));

        vec
    }
}

impl From<(MessageId, Vec<Attempt>)> for AttemptCollection {
    fn from(value: (MessageId, Vec<Attempt>)) -> Self {
        Self {
            message_id: value.0,
            attempts: value.1,
        }
    }
}

pub struct AttemptLog {
    #[allow(dead_code)]
    attempt_id: AttemptId,
    #[allow(dead_code)]
    processing_time: Duration,
    #[allow(dead_code)]
    response_time: Duration,
    #[allow(dead_code)]
    response_body: Option<String>,
}

impl AttemptLog {
    #[must_use]
    pub fn new(
        attempt_id: AttemptId,
        processing_time: Duration,
        response_time: Duration,
        response_body: Option<String>,
    ) -> Self {
        Self {
            attempt_id,
            processing_time,
            response_time,
            response_body,
        }
    }
}

#[cfg(test)]
mod message_test {
    use chrono::{DateTime, Utc};
    use serde_json::json;
    use test_case::test_case;

    use crate::configuration::domain::Topic;
    use crate::events::domain::{Event, Payload};
    use crate::tests::dt;
    use crate::time::Clock::Fixed;
    use crate::types::ApplicationId;

    #[test]
    #[should_panic(
        expected = "Unable to calculate processing time because created_at_date is after now date"
    )]
    fn processing_time_cannot_be_in_future() {
        let created_at = dt!("2014-11-28T12:00:09Z");
        let now = dt!("2014-11-28T12:00:08Z");

        let sut = MessageObjectMother::with_created_at_str(created_at);

        let _ = sut.calculate_processing_time(&Fixed(now));
    }

    #[test_case("2014-11-28T12:00:09Z", "2014-11-28T12:00:10Z", 1000; "1 sec")]
    #[test_case("2014-11-28T12:00:09Z", "2014-11-28T12:00:09.425Z", 425; "425 ms")]
    #[test_case("2014-11-28T12:00:09Z", "2014-11-28T12:01:12.997Z", 63_997; "1 min")]
    fn processing_time(created_at: &str, now: &str, expected_id_ms: u128) {
        let created_at = dt!(created_at);
        let now = dt!(now);

        let sut = MessageObjectMother::with_created_at_str(created_at);

        let processing_time = sut.calculate_processing_time(&Fixed(now));

        assert_eq!(expected_id_ms, processing_time.as_millis());
    }

    struct MessageObjectMother;

    impl MessageObjectMother {
        fn with_created_at_str(created_at: DateTime<Utc>) -> Event {
            let clock = Fixed(created_at);

            Event::new(
                ApplicationId::new(),
                Payload::from(json!({"foo": "bar"})),
                Topic::new("contact.created").unwrap(),
                &clock,
            )
        }
    }
}

#[cfg(test)]
mod attempt_test {
    use test_case::test_case;

    use crate::events::domain::Attempt;
    use crate::sender::Status;
    use crate::types::{AttemptId, MessageId};

    #[test]
    #[should_panic]
    fn attempt_id_should_be_greater_than_0() {
        let attempt_id = AttemptId::new(MessageId::new(), 0).unwrap();

        Attempt::new(attempt_id, Status::Numeric(200));
    }

    #[test_case(Status::Numeric(200), true)]
    #[test_case(Status::Numeric(201), true)]
    #[test_case(Status::Numeric(299), true)]
    #[test_case(Status::Numeric(300), false)]
    #[test_case(Status::Numeric(400), false)]
    #[test_case(Status::Numeric(502), false)]
    #[test_case(Status::Unknown("test".to_string()), false)]
    fn attempt_is_delivered(status: Status, expected: bool) {
        let attempt_id = AttemptId::new(MessageId::new(), 1).unwrap();
        let sut = Attempt::new(attempt_id, status);

        assert_eq!(expected, sut.is_delivered());
    }
}

#[cfg(test)]
mod attempt_collection_test {
    use crate::events::domain::AttemptCollection;
    use crate::sender::Status::Numeric;
    use crate::types::MessageId;

    #[test]
    fn get_attempts_from_collection() {
        let mut sut = AttemptCollection::new(MessageId::new());

        sut.push(Numeric(504));
        sut.push(Numeric(502));
        sut.push(Numeric(500));
        sut.push(Numeric(400));
        sut.push(Numeric(200));

        let mut vec = sut.all().into_iter();

        assert_eq!(Numeric(504), vec.next().unwrap().status);
        assert_eq!(Numeric(502), vec.next().unwrap().status);
        assert_eq!(Numeric(500), vec.next().unwrap().status);
        assert_eq!(Numeric(400), vec.next().unwrap().status);
        assert_eq!(Numeric(200), vec.next().unwrap().status);
        assert_eq!(None, vec.next());
    }

    #[test]
    #[should_panic(expected = "Could not push to the attempt collection when was delivered")]
    fn cannot_push_attempt_when_collection_is_delivered() {
        let mut sut = AttemptCollection::new(MessageId::new());

        sut.push(Numeric(200));
        sut.push(Numeric(200));
    }

    #[test]
    fn should_have_ordered_unique_attempts() {
        let mut sut = AttemptCollection::new(MessageId::new());

        sut.push(Numeric(500));
        sut.push(Numeric(501));
        sut.push(Numeric(502));
        sut.push(Numeric(200));

        let vec = sut.all();
        let mut iter = vec.iter();

        assert_eq!(1, iter.next().unwrap().id.attempt_no());
        assert_eq!(2, iter.next().unwrap().id.attempt_no());
        assert_eq!(3, iter.next().unwrap().id.attempt_no());
        assert_eq!(4, iter.next().unwrap().id.attempt_no());
    }
}
