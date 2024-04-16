use serde::{Deserialize, Serialize};

use crate::configuration::domain::EndpointId;
use crate::events::domain::MessageId;

#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum AsyncMessage {
    SentMessage(SentMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SentMessage {
    msg_id: String,
    pub attempt: usize,
    endpoint_id: String,
}

impl SentMessage {
    pub fn new(msg_id: MessageId, endpoint_id: EndpointId) -> Self {
        Self {
            msg_id: msg_id.to_string(),
            attempt: 1,
            endpoint_id: endpoint_id.to_string(),
        }
    }

    pub fn with_increased_attempt(&self) -> SentMessage {
        Self {
            msg_id: self.msg_id.clone(),
            attempt: self.attempt + 1,
            endpoint_id: self.endpoint_id.clone(),
        }
    }

    pub fn msg_id(&self) -> MessageId {
        MessageId::try_from(self.msg_id.clone()).unwrap()
    }

    pub fn endpoint_id(&self) -> EndpointId {
        EndpointId::try_from(self.endpoint_id.clone()).unwrap()
    }
}
