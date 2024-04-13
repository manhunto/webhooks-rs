use std::collections::HashMap;

use log::debug;

pub enum State {
    Close,
    Open,
}

pub struct CircuitBreaker {
    storage: HashMap<String, u32>, // todo extract trait, allow to persist in redis
}

impl CircuitBreaker {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn call<T, E>(&mut self, key: String, result: Result<T, E>) -> State {
        match result {
            Ok(_) => {
                self.storage.entry(key.clone()).or_insert(0);

                State::Open
            }
            Err(_) => {
                *self.storage.entry(key.clone()).or_insert(0) += 1;

                if let Some(fail_count) = self.storage.get(&key) {
                    debug!("Service {} current fail count: {}", key, fail_count);

                    if fail_count.ge(&5) {
                        debug!("Service {} reached a limit and is closed", key);

                        return State::Close;
                    }
                }

                State::Open
            }
        }
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new()
    }
}