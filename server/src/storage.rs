use sqlx::PgPool;

use crate::configuration::storage::{ApplicationStorage, EndpointStorage};
use crate::events::storage::{
    AttemptLogStorage, EventStorage, InMemoryAttemptLogStorage, MessageStorage,
};

pub struct Storage {
    pub applications: ApplicationStorage,
    pub endpoints: EndpointStorage,
    pub events: EventStorage,
    pub messages: MessageStorage,
    pub attempt_log: Box<dyn AttemptLogStorage + Sync + Send>,
}

impl Storage {
    pub fn new(pool: PgPool) -> Self {
        Self {
            applications: ApplicationStorage::new(pool.clone()),
            endpoints: EndpointStorage::new(pool.clone()),
            events: EventStorage::new(pool.clone()),
            messages: MessageStorage::new(pool),
            attempt_log: Box::new(InMemoryAttemptLogStorage::new()),
        }
    }
}
