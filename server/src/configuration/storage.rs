use crate::configuration::domain::{Application, ApplicationId, Endpoint};
use crate::error::Error;
use crate::error::Error::EntityNotFound;
use std::sync::Mutex;

pub trait ApplicationStorage {
    fn save(&self, app: Application);

    fn count(&self) -> usize;

    fn exists(&self, app_id: &ApplicationId) -> bool;

    fn get(&self, app_id: &ApplicationId) -> Result<Application, Error>;
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

    fn exists(&self, app_id: &ApplicationId) -> bool {
        let applications = self.applications.lock().unwrap();

        applications.iter().any(|app| app.id.eq(app_id))
    }

    fn get(&self, app_id: &ApplicationId) -> Result<Application, Error> {
        let applications = self.applications.lock().unwrap();

        applications
            .clone()
            .into_iter()
            .find(|app| app.id.eq(app_id))
            .ok_or_else(|| EntityNotFound("Application not found".to_string()))
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
