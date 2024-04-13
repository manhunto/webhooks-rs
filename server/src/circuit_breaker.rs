use std::collections::HashMap;

use log::debug;

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Close,
    Open,
}

// todo extract policy
pub struct CircuitBreaker {
    storage: HashMap<String, u32>,
    // todo extract trait, allow to persist in redis,
    states: HashMap<String, State>,
}

impl CircuitBreaker {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            states: HashMap::new(),
        }
    }

    pub fn is_call_permitted(&self, key: String) -> bool {
        self.states.get(&key).unwrap_or(&State::Open) == &State::Close
    }

    pub fn call<T, E>(&mut self, key: String, result: Result<T, E>) -> State {
        if self.is_call_permitted(key.clone()) {
            debug!("Service {} closed", key);

            return State::Close;
        }

        match result {
            Ok(_) => {
                self.storage.entry(key.clone()).or_insert(0);

                self.update_and_return(key, State::Open)
            }
            Err(_) => {
                *self.storage.entry(key.clone()).or_insert(0) += 1;

                if let Some(fail_count) = self.storage.get(&key) {
                    debug!("Service {} current fail count: {}", key, fail_count);

                    if fail_count.ge(&3) {
                        debug!("Service {} reached a limit and is closed", key);

                        return self.update_and_return(key, State::Close);
                    }
                }

                self.update_and_return(key, State::Open)
            }
        }
    }

    fn update_and_return(&mut self, key: String, state: State) -> State {
        *self.states.entry(key).or_insert(State::Open) = state;

        state
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new()
    }
}
