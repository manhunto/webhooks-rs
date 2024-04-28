use serde::{Deserialize, Serialize};

use crate::events::domain::RoutedMessageId;

#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum AsyncMessage {
    SentMessage(SentMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SentMessage {
    routed_msg_id: String,
    pub attempt: usize,
}

impl SentMessage {
    pub fn new(routed_message_id: RoutedMessageId) -> Self {
        Self {
            routed_msg_id: routed_message_id.to_string(),
            attempt: 1,
        }
    }

    pub fn with_increased_attempt(&self) -> SentMessage {
        Self {
            routed_msg_id: self.routed_msg_id.clone(),
            attempt: self.attempt + 1,
        }
    }

    pub fn routed_msg_id(&self) -> RoutedMessageId {
        RoutedMessageId::try_from(self.routed_msg_id.clone()).unwrap()
    }
}
