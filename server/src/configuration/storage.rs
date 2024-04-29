use std::collections::HashMap;
use std::sync::Mutex;

use crate::configuration::domain::{Application, Endpoint, Topic};
use crate::error::Error;
use crate::error::Error::EntityNotFound;
use crate::types::{ApplicationId, EndpointId};

pub trait ApplicationStorage {
    fn save(&self, app: Application);

    fn count(&self) -> usize;

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

    fn for_topic(&self, application_id: &ApplicationId, topic: &Topic) -> Vec<Endpoint>;

    fn get(&self, endpoint_id: &EndpointId) -> Result<Endpoint, Error>;
}

pub struct InMemoryEndpointStorage {
    endpoints: Mutex<HashMap<String, Endpoint>>,
}

impl InMemoryEndpointStorage {
    pub fn new() -> Self {
        Self {
            endpoints: Mutex::new(HashMap::new()),
        }
    }
}

impl EndpointStorage for InMemoryEndpointStorage {
    fn save(&self, endpoint: Endpoint) {
        let mut endpoints = self.endpoints.lock().unwrap();

        endpoints.insert(endpoint.id.to_string(), endpoint.clone());
    }

    fn count(&self) -> usize {
        let endpoints = self.endpoints.lock().unwrap();

        endpoints.len()
    }

    fn for_topic(&self, application_id: &ApplicationId, topic: &Topic) -> Vec<Endpoint> {
        let endpoints = self.endpoints.lock().unwrap();

        endpoints
            .values()
            .clone()
            .filter(|endpoint| {
                endpoint.app_id.eq(application_id) && endpoint.topics.contains(topic)
            })
            .cloned()
            .collect()
    }

    fn get(&self, endpoint_id: &EndpointId) -> Result<Endpoint, Error> {
        let endpoints = self.endpoints.lock().unwrap();

        endpoints
            .values()
            .find(|endpoint| endpoint.id.eq(endpoint_id))
            .ok_or_else(|| EntityNotFound("Endpoint not found".to_string()))
            .cloned()
    }
}
