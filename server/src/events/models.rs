use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub payload: Payload,
}

#[derive(Debug)]
pub struct Payload {
    #[allow(dead_code)]
    value: String,
}

impl<'de> Deserialize<'de> for Payload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let payload: Box<RawValue> = Deserialize::deserialize(deserializer)?;

        Ok(Self {
            value: payload.to_string(),
        })
    }
}
