use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::events::domain::Event;

#[derive(Deserialize)]
pub struct CreateEventRequest {
    pub payload: Value,
    pub topic: String,
}

#[derive(Serialize)]
pub struct CreateEventResponse {
    id: String,
}

impl From<Event> for CreateEventResponse {
    fn from(value: Event) -> Self {
        Self {
            id: value.id.to_string(),
        }
    }
}
