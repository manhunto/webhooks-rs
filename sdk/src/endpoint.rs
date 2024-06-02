use serde::Deserialize;
use serde_json::json;

use crate::client::{Client, EndpointUrl};
use crate::error::Error;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Endpoint {
    pub id: String,
    pub app_id: String,
    pub url: String,
    pub topics: Vec<String>,
}

pub struct EndpointApi {
    client: Client,
}

impl EndpointApi {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        app_id: &str,
        url: &str,
        topics: Vec<&str>,
    ) -> Result<Endpoint, Error> {
        let body = json!({
            "url": url,
            "topics": topics
        });

        self.client
            .post(
                EndpointUrl::try_from(format!("application/{}/endpoint", app_id)).unwrap(),
                body,
            )
            .await
    }
}
