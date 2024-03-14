use crate::endpoint::domain::Endpoint;
use std::sync::Mutex;

pub trait EndpointStorage {
    fn save(&self, endpoint: Endpoint);

    fn count(&self) -> usize;
}

pub struct InMemoryEndpointStorage {
    endpoints: Mutex<Vec<Endpoint>>,
}

impl InMemoryEndpointStorage {
    pub fn new() -> Self {
        Self {
            endpoints: Mutex::new(vec![]),
        }
    }
}

impl EndpointStorage for InMemoryEndpointStorage {
    fn save(&self, app: Endpoint) {
        let mut endpoints = self.endpoints.lock().unwrap();

        endpoints.push(app);
    }

    fn count(&self) -> usize {
        let endpoints = self.endpoints.lock().unwrap();

        endpoints.len()
    }
}
