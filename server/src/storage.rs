use crate::configuration::storage::{
    ApplicationStorage, EndpointStorage, InMemoryApplicationStorage, InMemoryEndpointStorage,
};
use crate::events::storage::{
    InMemoryMessageStorage, InMemoryRoutedMessageStorage, MessageStorage, RoutedMessageStorage,
};

pub struct Storage {
    pub applications: Box<dyn ApplicationStorage + Sync + Send>,
    pub endpoints: Box<dyn EndpointStorage + Sync + Send>,
    pub messages: Box<dyn MessageStorage + Sync + Send>,
    pub routed_messages: Box<dyn RoutedMessageStorage + Sync + Send>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            applications: Box::new(InMemoryApplicationStorage::new()),
            endpoints: Box::new(InMemoryEndpointStorage::new()),
            messages: Box::new(InMemoryMessageStorage::new()),
            routed_messages: Box::new(InMemoryRoutedMessageStorage::new()),
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}
