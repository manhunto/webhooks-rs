use sqlx::PgPool;

use crate::configuration::storage::{ApplicationStorage, EndpointStorage};
use crate::events::storage::{
    AttemptLogStorage, EventStorage, InMemoryAttemptLogStorage, InMemoryRoutedMessageStorage,
    RoutedMessageStorage,
};

pub struct Storage {
    pub applications: ApplicationStorage,
    pub endpoints: EndpointStorage,
    pub events: EventStorage,
    pub routed_messages: Box<dyn RoutedMessageStorage + Sync + Send>,
    pub attempt_log: Box<dyn AttemptLogStorage + Sync + Send>,
}

impl Storage {
    pub fn new(pool: PgPool) -> Self {
        Self {
            applications: ApplicationStorage::new(pool.clone()),
            endpoints: EndpointStorage::new(pool.clone()),
            events: EventStorage::new(pool),
            routed_messages: Box::new(InMemoryRoutedMessageStorage::new()),
            attempt_log: Box::new(InMemoryAttemptLogStorage::new()),
        }
    }
}
