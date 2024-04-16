use std::sync::Mutex;

use crate::error::Error;
use crate::error::Error::EntityNotFound;
use crate::events::domain::{Message, MessageId};

pub trait MessageStorage {
    fn save(&self, app: Message);

    fn count(&self) -> usize;

    fn get(&self, message_id: MessageId) -> Result<Message, Error>;
}

pub struct InMemoryMessageStorage {
    messages: Mutex<Vec<Message>>,
}

impl InMemoryMessageStorage {
    pub fn new() -> Self {
        Self {
            messages: Mutex::new(vec![]),
        }
    }
}

impl MessageStorage for InMemoryMessageStorage {
    fn save(&self, app: Message) {
        let mut messages = self.messages.lock().unwrap();

        messages.push(app);
    }

    fn count(&self) -> usize {
        let messages = self.messages.lock().unwrap();

        messages.len()
    }

    fn get(&self, message_id: MessageId) -> Result<Message, Error> {
        let messages = self.messages.lock().unwrap();

        messages
            .clone()
            .into_iter()
            .find(|msg| msg.id.eq(&message_id))
            .ok_or_else(|| EntityNotFound("Message not found".to_string()))
    }
}
