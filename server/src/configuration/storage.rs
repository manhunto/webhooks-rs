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

    pub async fn get(&self, app_id: &ApplicationId) -> Result<Application, Error> {
        let record = query!(
            r#"
            SELECT * FROM applications WHERE id = $1
        "#,
            app_id.to_base62()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Application {
            name: record.name,
            id: ApplicationId::try_from(format!("app_{}", record.id)).unwrap(), // fixme: without adding prefix
        })
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
        query!(
            r#"
        INSERT INTO endpoints (id, app_id, url, topics, status)
        VALUES ($1, $2, $3, $4, $5)
        "#,
            endpoint.id.to_base62(),
            endpoint.app_id.to_base62(),
            endpoint.url.to_string(),
            json!(endpoint.topics.as_strings()),
            endpoint.status.to_string()
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn for_topic(&self, application_id: &ApplicationId, topic: &Topic) -> Vec<Endpoint> {
        let endpoints = query_as::<_, Endpoint>(
            r#"
            SELECT * FROM endpoints WHERE app_id = $1
        "#,
        )
        .bind(application_id.to_base62())
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
            r#"
            SELECT * FROM endpoints WHERE id = $1
        "#,
        )
        .bind(endpoint_id.to_base62())
        .fetch_one(&self.pool)
        .await?)
    }
}
