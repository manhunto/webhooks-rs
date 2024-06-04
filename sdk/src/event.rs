use serde::Deserialize;
use serde_json::{json, Value};

use crate::client::{Client, EndpointUrl};
use crate::error::Error;

#[derive(Deserialize, Debug, PartialEq)]
pub struct CreateEventResponse {
    pub id: String,
}

pub struct EventsApi {
    client: Client,
}

impl EventsApi {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        app_id: &str,
        topic: &str,
        payload: &Value,
    ) -> Result<CreateEventResponse, Error> {
        let body = json!({
            "topic": topic,
            "payload": payload
        });

        self.client
            .post(
                EndpointUrl::try_from(format!("application/{}/event", app_id)).unwrap(),
                body,
            )
            .await
    }
}
