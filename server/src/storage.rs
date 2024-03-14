use crate::application::storage::{ApplicationStorage, InMemoryApplicationStorage};
use crate::endpoint::storage::{EndpointStorage, InMemoryEndpointStorage};

pub struct Storage {
    pub applications: Box<dyn ApplicationStorage + Sync + Send>,
    pub endpoints: Box<dyn EndpointStorage + Sync + Send>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            applications: Box::new(InMemoryApplicationStorage::new()),
            endpoints: Box::new(InMemoryEndpointStorage::new()),
        }
    }
}
