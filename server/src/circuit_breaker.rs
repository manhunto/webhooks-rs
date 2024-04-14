use std::collections::HashMap;

use log::debug;

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Closed,
    Open,
}

pub enum Error<T> {
    Rejected,
    Open(T),
    Closed(T),
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
        self.states.get(&key).unwrap_or(&State::Open) == &State::Closed
    }

    pub fn call<T, E, F>(&mut self, key: String, function: F) -> Result<T, Error<E>>
    where
        F: FnOnce() -> Result<T, E>,
    {
        if self.is_call_permitted(key.clone()) {
            debug!("Service {} closed", key);

            return Err(Error::Rejected);
        }

        match function() {
            Ok(ok) => {
                self.storage.entry(key.clone()).or_insert(0);

                Ok(ok)
            }
            Err(err) => {
                *self.storage.entry(key.clone()).or_insert(0) += 1;

                if let Some(fail_count) = self.storage.get(&key) {
                    debug!("Service {} current fail count: {}", key, fail_count);

                    if fail_count.ge(&3) {
                        debug!("Service {} reached a limit and is closed", key);

                        self.update(key, State::Closed);

                        return Err(Error::Closed(err));
                    }
                }

                Err(Error::Open(err))
            }
        }
    }

    fn update(&mut self, key: String, state: State) {
        *self.states.entry(key).or_insert(State::Open) = state
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new()
    }
}
