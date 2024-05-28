use std::sync::Mutex;

use serde_json::json;
use sqlx::{query, query_as, FromRow, PgPool, Row};

use crate::error::Error;
use crate::events::domain::{Attempt, AttemptCollection, AttemptLog, Event, Message};
use crate::sender::Status;
use crate::types::{EndpointId, EventId, MessageId};

pub struct EventStorage {
    pool: PgPool,
}

impl EventStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, event: Event) {
        query(
            r#"
            INSERT INTO events (id, app_id, payload, topic, created_at)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        )
        .bind(event.id)
        .bind(event.app_id)
        .bind(json!(event.payload))
        .bind(event.topic.to_string())
        .bind(event.created_at.naive_utc())
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

pub struct MessageStorage {
    pool: PgPool,
}

impl MessageStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, message: Message) {
        let mut tx = self.pool.begin().await.unwrap();

        query(
            r#"
            INSERT INTO messages (id, event_id, endpoint_id)
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
        "#,
        )
        .bind(message.id)
        .bind(message.event_id)
        .bind(message.endpoint_id)
        .execute(&mut *tx)
        .await
        .unwrap();

        // todo optimize
        for attempt in message.attempts() {
            query(
                r#"
            INSERT INTO attempts (message_id, attempt, status_numeric, status_unknown)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
        "#,
            )
            .bind(attempt.message_id())
            .bind(attempt.attempt_id() as i16)
            .bind(match attempt.status() {
                Status::Numeric(val) => Some(val as i16),
                Status::Unknown(_) => None,
            })
            .bind(match attempt.status() {
                Status::Numeric(_) => None,
                Status::Unknown(val) => Some(val),
            })
            .execute(&mut *tx)
            .await
            .unwrap();
        }

        tx.commit().await.unwrap();
    }

    pub async fn get(&self, message_id: MessageId) -> Result<Message, Error> {
        let row = query(
            r#"
            SELECT * FROM messages WHERE id = $1
        "#,
        )
        .bind(message_id)
        .fetch_one(&self.pool)
        .await?;

        let event_id: EventId = row.try_get("event_id")?;
        let endpoint_id: EndpointId = row.try_get("endpoint_id")?;

        let attempt_rows = query(
            r#"
            SELECT * FROM attempts WHERE message_id = $1
        "#,
        )
        .bind(message_id)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        let attempts: Vec<Attempt> = attempt_rows
            .iter()
            .map(|p| Attempt::from_row(p).unwrap())
            .collect();
        let collection = AttemptCollection::from((message_id, attempts));

        Ok(Message {
            id: message_id,
            endpoint_id,
            event_id,
            attempts: collection,
        })
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
