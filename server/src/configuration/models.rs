use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::configuration::domain::{Application, Endpoint, Topic};

fn is_not_empty(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();

    if value.is_empty() {
        return Err(ValidationError::new("is_empty"));
    }

    Ok(())
}

fn topic_are_valid(value: &Vec<String>) -> Result<(), ValidationError> {
    for v in value {
        if Topic::try_from(v.as_str()).is_err() {
            let err = ValidationError::new("invalid_topic_name")
                .with_message(format!("'{}' is invalid topic name", v).into());

            return Err(err);
        }
    }

    Ok(())
}

#[derive(Deserialize, Validate)]
pub struct CreateAppRequest {
    #[validate(custom(function = is_not_empty, message = "Name cannot be empty"))]
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

#[derive(Deserialize, Validate)]
pub struct CreateEndpointRequest {
    #[validate(url(message = "Url should be valid"))]
    pub url: String,
    #[validate(length(min = 1, message = "Should be at leas one topic"))]
    #[validate(custom(function = topic_are_valid))]
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
            topics: value.topics.into(),
        }
    }
}
