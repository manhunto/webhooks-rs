use std::sync::Mutex;

use crate::error::Error;
use crate::error::Error::EntityNotFound;
use crate::events::domain::{Message, MessageId, RoutedMessage, RoutedMessageId};

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

pub trait RoutedMessageStorage {
    fn save(&self, routed_message: RoutedMessage);

    fn get(&self, routed_message_id: RoutedMessageId) -> Result<RoutedMessage, Error>;
}

pub struct InMemoryRoutedMessageStorage {
    data: Mutex<Vec<RoutedMessage>>,
}

impl InMemoryRoutedMessageStorage {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(vec![]),
        }
    }
}

impl RoutedMessageStorage for InMemoryRoutedMessageStorage {
    fn save(&self, routed_message: RoutedMessage) {
        let mut data = self.data.lock().unwrap();

        data.push(routed_message);
    }

    fn get(&self, routed_message_id: RoutedMessageId) -> Result<RoutedMessage, Error> {
        let data = self.data.lock().unwrap();

        data.clone()
            .into_iter()
            .find(|routed_message| routed_message.id.eq(&routed_message_id))
            .ok_or_else(|| EntityNotFound("Routed message not found".to_string()))
    }
}
