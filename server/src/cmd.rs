use serde::{Deserialize, Serialize};
use url::Url;

use crate::events::domain::{MessageId, Payload};

#[derive(Serialize, Deserialize, Debug)]
pub struct SentMessage {
    pub payload: String,
    pub url: String,
    pub msg_id: String,
    pub attempt: usize,
}

impl SentMessage {
    pub fn new(payload: Payload, url: Url, msg_id: MessageId) -> Self {
        Self {
            payload: payload.to_string(),
            url: url.to_string(),
            msg_id: msg_id.to_string(),
            attempt: 1,
        }
    }

    pub fn with_increased_attempt(&self) -> Self {
        Self {
            payload: self.payload.clone(),
            url: self.url.clone(),
            msg_id: self.msg_id.clone(),
            attempt: self.attempt + 1,
        }
    }
}
