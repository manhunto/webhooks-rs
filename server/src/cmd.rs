use serde::{Deserialize, Serialize};

use crate::types::MessageId;

#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum AsyncMessage {
    SentMessage(SentMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SentMessage {
    msg_id: String,
    pub attempt: usize,
}

impl SentMessage {
    pub fn new(message_id: MessageId) -> Self {
        Self {
            msg_id: message_id.to_string(),
            attempt: 1,
        }
    }

    pub fn with_increased_attempt(&self) -> SentMessage {
        Self {
            msg_id: self.msg_id.clone(),
            attempt: self.attempt + 1,
        }
    }

    pub fn msg_id(&self) -> MessageId {
        MessageId::try_from(self.msg_id.clone()).unwrap()
    }
}
