use serde_json::json;
use sqlx::{query, query_as, PgPool};

use crate::configuration::domain::{Application, Endpoint, Topic};
use crate::error::Error;
use crate::types::{ApplicationId, EndpointId};

pub struct ApplicationStorage {
    pool: PgPool,
}

impl ApplicationStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, app: Application) {
        query(
            r"
            INSERT INTO applications (id, name)
            VALUES ($1, $2)
        ",
        )
        .bind(app.id)
        .bind(app.name)
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn get(&self, app_id: &ApplicationId) -> Result<Application, Error> {
        Ok(query_as::<_, Application>(
            r"
            SELECT * FROM applications WHERE id = $1
            ",
        )
        .bind(app_id)
        .fetch_one(&self.pool)
        .await?)
    }
}

pub struct EndpointStorage {
    pool: PgPool,
}

impl EndpointStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, endpoint: Endpoint) {
        query(
            r"
        INSERT INTO endpoints (id, app_id, url, topics, status)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (id) DO UPDATE 
            SET url = EXCLUDED.url,
                topics = EXCLUDED.topics,
                status = EXCLUDED.status
        ",
        )
        .bind(endpoint.id)
        .bind(endpoint.app_id)
        .bind(endpoint.url.to_string())
        .bind(json!(endpoint.topics.as_strings()))
        .bind(endpoint.status.to_string())
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn for_topic(&self, application_id: &ApplicationId, topic: &Topic) -> Vec<Endpoint> {
        let endpoints = query_as::<_, Endpoint>(
            r"
            SELECT * FROM endpoints WHERE app_id = $1
        ",
        )
        .bind(application_id)
        .fetch_all(&self.pool)
        .await
        .expect("Error in query");

        endpoints
            .into_iter()
            .filter(|e| e.topics.contains(topic))
            .collect() // todo: add it to the query
    }

    pub async fn get(&self, endpoint_id: &EndpointId) -> Result<Endpoint, Error> {
        Ok(query_as::<_, Endpoint>(
            r"
            SELECT * FROM endpoints WHERE id = $1
        ",
        )
        .bind(endpoint_id)
        .fetch_one(&self.pool)
        .await?)
    }
}
