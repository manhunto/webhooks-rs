use std::sync::Mutex;

use crate::error::Error;
use crate::error::Error::EntityNotFound;
use crate::events::domain::{AttemptLog, Event, RoutedMessage};
use crate::types::{EventId, RoutedMessageId};

pub trait MessageStorage {
    fn save(&self, app: Event);

    fn get(&self, event_id: EventId) -> Result<Event, Error>;
}

pub struct InMemoryMessageStorage {
    events: Mutex<Vec<Event>>,
}

impl InMemoryMessageStorage {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(vec![]),
        }
    }
}

impl MessageStorage for InMemoryMessageStorage {
    fn save(&self, app: Event) {
        self.events.lock().unwrap().push(app);
    }

    fn get(&self, event_id: EventId) -> Result<Event, Error> {
        self.events
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .find(|event| event.id.eq(&event_id))
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

pub trait AttemptLogStorage {
    fn save(&self, attempt_log: AttemptLog);
}

pub struct InMemoryAttemptLogStorage {
    data: Mutex<Vec<AttemptLog>>,
}

impl InMemoryAttemptLogStorage {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(vec![]),
        }
    }
}

impl AttemptLogStorage for InMemoryAttemptLogStorage {
    fn save(&self, attempt_log: AttemptLog) {
        let mut data = self.data.lock().unwrap();

        data.push(attempt_log);
    }
}
