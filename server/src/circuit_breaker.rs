use std::collections::HashMap;
use std::future::Future;

use log::debug;

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Closed,
    Open,
}

#[derive(PartialEq, Debug)]
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

    // todo: key can be AsRef<str>
    pub async fn call<T, E, F, Fut>(&mut self, key: &String, function: F) -> Result<T, Error<E>>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        if self.is_call_permitted(key.clone()) {
            debug!("Service {} closed", key);

            return Err(Error::Rejected);
        }

        match function().await {
            Ok(ok) => {
                self.reset_counter(key);

                Ok(ok)
            }
            Err(err) => {
                *self.storage.entry(key.clone()).or_insert(0) += 1;

                if let Some(fail_count) = self.storage.get(key) {
                    debug!("Service {} current fail count: {}", key, fail_count);

                    if fail_count.ge(&3) {
                        debug!("Service {} reached a limit and is closed", key);

                        self.update(key.clone(), State::Closed);

                        return Err(Error::Closed(err));
                    }
                }

                Err(Error::Open(err))
            }
        }
    }

    pub fn revive(&mut self, key: &str) {
        if self.state(key.to_string()) == &State::Closed {
            self.reset_counter(key);
            self.update(key.to_owned(), State::Open);
        }
    }

    fn reset_counter(&mut self, key: &str) {
        *self.storage.entry(key.to_owned()).or_insert(0) = 0;
    }

    fn is_call_permitted(&self, key: String) -> bool {
        self.state(key) == &State::Closed
    }

    fn state(&self, key: String) -> &State {
        self.states.get(&key).unwrap_or(&State::Open)
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

#[cfg(test)]
mod tests {
    use crate::circuit_breaker::CircuitBreaker;
    use crate::circuit_breaker::Error::{Closed, Open, Rejected};

    #[tokio::test]
    async fn successful_calls_doesnt_close_the_endpoint() {
        let mut sut = CircuitBreaker::new();
        let key = "key".to_string();

        assert_eq!(Ok(0), sut.call(&key, ok).await);
        assert_eq!(Ok(0), sut.call(&key, ok).await);
        assert_eq!(Ok(0), sut.call(&key, ok).await);
        assert_eq!(Ok(0), sut.call(&key, ok).await);
        assert_eq!(Ok(0), sut.call(&key, ok).await);
        assert_eq!(Ok(0), sut.call(&key, ok).await);
        assert_eq!(Ok(0), sut.call(&key, ok).await);
        assert_eq!(Ok(0), sut.call(&key, ok).await);
        assert_eq!(Ok(0), sut.call(&key, ok).await);
    }

    #[tokio::test]
    async fn erroneous_calls_close_the_endpoint() {
        let mut sut = CircuitBreaker::new();
        let key = "key".to_string();

        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Closed(255)), sut.call(&key, err).await);
    }

    #[tokio::test]
    async fn calls_are_rejected_to_closed_endpoint() {
        let mut sut = CircuitBreaker::new();
        let key = "key".to_string();

        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Closed(255)), sut.call(&key, err).await);

        assert_eq!(Err(Rejected), sut.call(&key, ok).await);
        assert_eq!(Err(Rejected), sut.call(&key, err).await);
    }

    #[tokio::test]
    async fn successful_call_resets_counter() {
        let mut sut = CircuitBreaker::new();
        let key = "key".to_string();

        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Ok(0), sut.call(&key, ok).await);

        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Ok(0), sut.call(&key, ok).await);
    }

    #[tokio::test]
    async fn every_key_has_own_counter() {
        let mut sut = CircuitBreaker::new();
        let key = "key".to_string();
        let key2 = "key2".to_string();

        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Closed(255)), sut.call(&key, err).await);

        assert_eq!(Err(Open(255)), sut.call(&key2, err).await);
        assert_eq!(Err(Open(255)), sut.call(&key2, err).await);
        assert_eq!(Err(Closed(255)), sut.call(&key2, err).await);
    }

    #[tokio::test]
    async fn revive_closed() {
        let mut sut = CircuitBreaker::new();
        let key = "key".to_string();

        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Closed(255)), sut.call(&key, err).await);

        sut.revive(&key);

        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Closed(255)), sut.call(&key, err).await);

        sut.revive(&key);

        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
    }

    #[tokio::test]
    async fn revive_opened_doesnt_reset_counter() {
        let mut sut = CircuitBreaker::new();
        let key = "key".to_string();

        assert_eq!(Err(Open(255)), sut.call(&key, err).await);
        assert_eq!(Err(Open(255)), sut.call(&key, err).await);

        sut.revive(&key);

        assert_eq!(Err(Closed(255)), sut.call(&key, err).await);
    }

    async fn ok() -> Result<u8, u8> {
        Ok(0)
    }

    async fn err() -> Result<u8, u8> {
        Err(255)
    }
}
