use crate::configuration::domain::{Application, Endpoint};
use std::sync::Mutex;

pub trait ApplicationStorage {
    fn save(&self, app: Application);

    fn count(&self) -> usize;
}

pub struct InMemoryApplicationStorage {
    applications: Mutex<Vec<Application>>,
}

impl InMemoryApplicationStorage {
    pub fn new() -> Self {
        Self {
            applications: Mutex::new(vec![]),
        }
    }
}

impl ApplicationStorage for InMemoryApplicationStorage {
    fn save(&self, app: Application) {
        let mut applications = self.applications.lock().unwrap();

        applications.push(app);
    }

    fn count(&self) -> usize {
        let applications = self.applications.lock().unwrap();

        applications.len()
    }
}

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
