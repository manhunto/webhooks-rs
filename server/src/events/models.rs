use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct CreateEventRequest {
    pub payload: Value,
    pub topic: String,
}
