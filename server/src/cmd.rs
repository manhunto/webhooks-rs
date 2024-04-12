use serde::{Deserialize, Serialize};
use url::Url;

use crate::configuration::domain::EndpointId;
use crate::events::domain::{MessageId, Payload};

#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum AsyncMessage {
    SentMessage(SentMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SentMessage {
    pub payload: String,
    pub url: String,
    pub msg_id: String,
    pub attempt: usize,
    pub endpoint_id: String,
}

impl SentMessage {
    pub fn new(payload: Payload, url: Url, msg_id: MessageId, endpoint_id: EndpointId) -> Self {
        Self {
            payload: payload.to_string(),
            url: url.to_string(),
            msg_id: msg_id.to_string(),
            attempt: 1,
            endpoint_id: endpoint_id.to_string(),
        }
    }

    pub fn with_increased_attempt(&self) -> SentMessage {
        Self {
            payload: self.payload.clone(),
            url: self.url.clone(),
            msg_id: self.msg_id.clone(),
            attempt: self.attempt + 1,
            endpoint_id: self.endpoint_id.clone(),
        }
    }
}
