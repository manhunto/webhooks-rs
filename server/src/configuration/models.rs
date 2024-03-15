use crate::configuration::domain::{Application, Endpoint};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateAppRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateAppResponse {
    id: String,
    name: String,
}

impl From<Application> for CreateAppResponse {
    fn from(value: Application) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateEndpointRequest {
    pub url: String,
    pub topics: Vec<String>,
}

#[derive(Serialize)]
pub struct CreateEndpointResponse {
    id: String,
    app_id: String,
    url: String,
    topics: Vec<String>,
}

impl From<Endpoint> for CreateEndpointResponse {
    fn from(value: Endpoint) -> Self {
        Self {
            id: value.id.to_string(),
            app_id: value.app_id.to_string(),
            url: value.url.to_string(),
            topics: value.topics.into_iter().map(|t| t.to_string()).collect(),
        }
    }
}
