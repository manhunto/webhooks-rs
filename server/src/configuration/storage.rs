use std::collections::HashMap;
use std::sync::Mutex;

use sqlx::{query, PgPool};

use crate::configuration::domain::{Application, Endpoint, Topic};
use crate::error::Error;
use crate::error::Error::EntityNotFound;
use crate::types::{ApplicationId, EndpointId};

pub struct ApplicationStorage {
    pool: PgPool,
}

impl ApplicationStorage {
    pub fn new(pool: PgPool) -> Self {
        ApplicationStorage { pool }
    }

    pub async fn save(&self, app: Application) {
        query!(
            r#"
            INSERT INTO applications (id, name)
            VALUES ($1, $2)
        "#,
            app.id.to_base62(),
            app.name
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn count(&self) -> usize {
        query!(
            r#"
            SELECT COUNT(*)
        "#
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
        .count
        .unwrap() as usize
    }

    pub async fn get(&self, app_id: &ApplicationId) -> Result<Application, Error> {
        let record = query!(
            r#"
            SELECT * FROM applications where id = $1
        "#,
            app_id.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Application {
            name: record.name,
            id: ApplicationId::try_from(format!("app_{}", record.id)).unwrap(), // fixme: without adding prefix
        })
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
