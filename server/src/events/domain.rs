use crate::configuration::domain::ApplicationId;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
#[allow(dead_code)] // todo remove me soon
pub struct Payload {
    body: String,
}

impl From<String> for Payload {
    fn from(value: String) -> Self {
        Self { body: value }
    }
}

#[derive(Debug, Clone, derive::Ksuid)]
#[prefix = "msg"]
struct MessageId {
    id: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // todo remove me soon
pub struct Message {
    id: MessageId,
    app_id: ApplicationId,
    payload: Payload,
}

impl Message {
    pub fn new(app_id: ApplicationId, payload: Payload) -> Self {
        Self {
            id: MessageId::new(),
            app_id,
            payload,
        }
    }
}
