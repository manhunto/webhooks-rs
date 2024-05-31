use sqlx::PgPool;

use crate::configuration::storage::{ApplicationStorage, EndpointStorage};
use crate::events::storage::{AttemptLogStorage, EventStorage, MessageStorage};

pub struct Storage {
    pub applications: ApplicationStorage,
    pub endpoints: EndpointStorage,
    pub events: EventStorage,
    pub messages: MessageStorage,
    pub attempt_log: AttemptLogStorage,
}

impl Storage {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self {
            applications: ApplicationStorage::new(pool.clone()),
            endpoints: EndpointStorage::new(pool.clone()),
            events: EventStorage::new(pool.clone()),
            messages: MessageStorage::new(pool.clone()),
            attempt_log: AttemptLogStorage::new(pool),
        }
    }
}
