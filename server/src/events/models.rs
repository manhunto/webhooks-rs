use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    #[serde(deserialize_with = "deserialize_json_string")]
    pub payload: String,
}

#[derive(Debug)]
pub struct Payload {
    #[allow(dead_code)]
    value: String,
}

fn deserialize_json_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Box<RawValue> = Deserialize::deserialize(deserializer)?;

    Ok(s.to_string())
}
