use std::sync::Mutex;

use serde_json::json;
use sqlx::{query, query_as, PgPool};

use crate::error::Error;
use crate::error::Error::EntityNotFound;
use crate::events::domain::{AttemptLog, Event, Message};
use crate::types::{EventId, MessageId};

pub struct EventStorage {
    pool: PgPool,
}

impl EventStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn save(&self, event: Event) {
        query!(
            r#"
            INSERT INTO events (id, app_id, payload, topic, created_at)
            VALUES ($1, $2, $3, $4, $5)
        "#,
            event.id.to_base62(),
            event.app_id.to_base62(),
            json!(event.payload),
            event.topic.to_string(),
            event.created_at.naive_utc()
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn get(&self, event_id: EventId) -> Result<Event, Error> {
        Ok(query_as::<_, Event>(
            r#"
            SELECT * FROM events WHERE id = $1
        "#,
        )
        .bind(event_id)
        .fetch_one(&self.pool)
        .await?)
    }
}

pub trait MessageStorage {
    fn save(&self, message: Message);

    fn get(&self, message_id: MessageId) -> Result<Message, Error>;
}

pub struct InMemoryMessageStorage {
    data: Mutex<Vec<Message>>,
}

impl InMemoryMessageStorage {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            data: Mutex::new(vec![]),
        }
    }
}

impl MessageStorage for InMemoryMessageStorage {
    fn save(&self, message: Message) {
        let mut data = self.data.lock().unwrap();

        data.push(message);
    }

    fn get(&self, message_id: MessageId) -> Result<Message, Error> {
        let data = self.data.lock().unwrap();

        data.clone()
            .into_iter()
            .find(|message| message.id.eq(&message_id))
            .ok_or_else(|| EntityNotFound("Message not found".to_string()))
    }
}

pub trait AttemptLogStorage {
    fn save(&self, attempt_log: AttemptLog);
}

pub struct InMemoryAttemptLogStorage {
    data: Mutex<Vec<AttemptLog>>,
}

impl InMemoryAttemptLogStorage {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            data: Mutex::new(vec![]),
        }
    }
}

impl AttemptLogStorage for InMemoryAttemptLogStorage {
    fn save(&self, attempt_log: AttemptLog) {
        let mut data = self.data.lock().unwrap();

        data.push(attempt_log);
    }
}
