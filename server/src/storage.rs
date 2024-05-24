use sqlx::PgPool;

use crate::configuration::storage::{ApplicationStorage, EndpointStorage};
use crate::events::storage::{
    AttemptLogStorage, EventStorage, InMemoryAttemptLogStorage, InMemoryMessageStorage,
    MessageStorage,
};

pub struct Storage {
    pub applications: ApplicationStorage,
    pub endpoints: EndpointStorage,
    pub events: EventStorage,
    pub messages: Box<dyn MessageStorage + Sync + Send>,
    pub attempt_log: Box<dyn AttemptLogStorage + Sync + Send>,
}

impl Storage {
    pub fn new(pool: PgPool) -> Self {
        Self {
            applications: ApplicationStorage::new(pool.clone()),
            endpoints: EndpointStorage::new(pool.clone()),
            events: EventStorage::new(pool),
            messages: Box::new(InMemoryMessageStorage::new()),
            attempt_log: Box::new(InMemoryAttemptLogStorage::new()),
        }
    }
}
