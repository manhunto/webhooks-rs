use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub payload: Value,
    pub topic: String,
}
