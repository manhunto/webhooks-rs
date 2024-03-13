use crate::application::storage::{ApplicationStorage, InMemoryApplicationStorage};

pub struct Storage {
    pub applications: Box<dyn ApplicationStorage + Sync + Send>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            applications: Box::new(InMemoryApplicationStorage::new()),
        }
    }
}
