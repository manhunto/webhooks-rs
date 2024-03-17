use crate::events::domain::Message;
use std::sync::Mutex;

pub trait MessageStorage {
    fn save(&self, app: Message);

    fn count(&self) -> usize;
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
}
