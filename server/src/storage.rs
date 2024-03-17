use crate::configuration::storage::{
    ApplicationStorage, EndpointStorage, InMemoryApplicationStorage, InMemoryEndpointStorage,
};
use crate::events::storage::{InMemoryMessageStorage, MessageStorage};

pub struct Storage {
    pub applications: Box<dyn ApplicationStorage + Sync + Send>,
    pub endpoints: Box<dyn EndpointStorage + Sync + Send>,
    pub messages: Box<dyn MessageStorage + Sync + Send>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            applications: Box::new(InMemoryApplicationStorage::new()),
            endpoints: Box::new(InMemoryEndpointStorage::new()),
            messages: Box::new(InMemoryMessageStorage::new()),
        }
    }
}
