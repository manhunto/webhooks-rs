use sqlx::PgPool;

use crate::configuration::storage::{ApplicationStorage, EndpointStorage, InMemoryEndpointStorage};
use crate::events::storage::{
    AttemptLogStorage, InMemoryAttemptLogStorage, InMemoryMessageStorage,
    InMemoryRoutedMessageStorage, MessageStorage, RoutedMessageStorage,
};

pub struct Storage {
    pub applications: ApplicationStorage,
    pub endpoints: Box<dyn EndpointStorage + Sync + Send>,
    pub messages: Box<dyn MessageStorage + Sync + Send>,
    pub routed_messages: Box<dyn RoutedMessageStorage + Sync + Send>,
    pub attempt_log: Box<dyn AttemptLogStorage + Sync + Send>,
}

impl Storage {
    pub fn new(pool: PgPool) -> Self {
        Self {
            applications: ApplicationStorage::new(pool),
            endpoints: Box::new(InMemoryEndpointStorage::new()),
            messages: Box::new(InMemoryMessageStorage::new()),
            routed_messages: Box::new(InMemoryRoutedMessageStorage::new()),
            attempt_log: Box::new(InMemoryAttemptLogStorage::new()),
        }
    }
}
