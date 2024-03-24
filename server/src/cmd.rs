use crate::events::domain::{MessageId, Payload};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct SentMessage {
    payload: String,
    url: String,
    msg_id: String,
}

impl SentMessage {
    pub fn new(payload: Payload, url: Url, msg_id: MessageId) -> Self {
        Self {
            payload: payload.to_string(),
            url: url.to_string(),
            msg_id: msg_id.to_string(),
        }
    }
}
