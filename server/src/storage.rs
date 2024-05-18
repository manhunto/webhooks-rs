use sqlx::PgPool;

use crate::configuration::storage::{ApplicationStorage, EndpointStorage};
use crate::events::storage::{
    AttemptLogStorage, InMemoryAttemptLogStorage, InMemoryMessageStorage,
    InMemoryRoutedMessageStorage, MessageStorage, RoutedMessageStorage,
};

pub struct Storage {
    pub applications: ApplicationStorage,
    pub endpoints: EndpointStorage,
    pub messages: Box<dyn MessageStorage + Sync + Send>,
    pub routed_messages: Box<dyn RoutedMessageStorage + Sync + Send>,
    pub attempt_log: Box<dyn AttemptLogStorage + Sync + Send>,
}

impl Storage {
    pub fn new(pool: PgPool) -> Self {
        Self {
            applications: ApplicationStorage::new(pool.clone()),
            endpoints: EndpointStorage::new(pool),
            messages: Box::new(InMemoryMessageStorage::new()),
            routed_messages: Box::new(InMemoryRoutedMessageStorage::new()),
            attempt_log: Box::new(InMemoryAttemptLogStorage::new()),
        }
    }
}
