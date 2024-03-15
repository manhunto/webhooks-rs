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
}

#[derive(Serialize)]
pub struct CreateEndpointResponse {
    id: String,
    app_id: String,
    url: String,
}

impl From<Endpoint> for CreateEndpointResponse {
    fn from(value: Endpoint) -> Self {
        Self {
            id: value.id.to_string(),
            app_id: value.app_id.to_string(),
            url: value.url.to_string(),
        }
    }
}
