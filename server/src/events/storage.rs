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
        let mut applications = self.messages.lock().unwrap();

        applications.push(app);
    }

    fn count(&self) -> usize {
        let applications = self.messages.lock().unwrap();

        applications.len()
    }
}
