use crate::endpoint::domain::Endpoint;
use serde::{Deserialize, Serialize};

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
