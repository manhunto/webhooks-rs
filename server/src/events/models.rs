use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub payload: Value,
    pub topic: String,
}

#[derive(Debug)]
pub struct Payload {
    #[allow(dead_code)]
    value: String,
}
