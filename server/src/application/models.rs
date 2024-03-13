use crate::application::domain::Application;
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
