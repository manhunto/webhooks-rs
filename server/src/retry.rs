use std::time::Duration;

use serde::Serialize;

pub trait Retryable<T: Serialize = Self>: Serialize {
    fn attempt(&self) -> usize;
    fn with_increased_attempt(&self) -> Self;
}

pub trait RetryPolicy {
    fn is_retryable(&self, retryable: &impl Retryable) -> bool;

    fn get_waiting_time(&self, retryable: &impl Retryable) -> Duration;
}

pub struct ExponentialRetryPolicy {
    max_retries: usize,
    multiplier: usize,
    delay: Duration,
}

impl ExponentialRetryPolicy {
    pub fn new(max_retries: usize, multiplier: usize, delay: Duration) -> Self {
        Self {
            max_retries,
            multiplier,
            delay,
        }
    }
}

impl RetryPolicy for ExponentialRetryPolicy {
    fn is_retryable(&self, retryable: &impl Retryable) -> bool {
        retryable.attempt() < self.max_retries
    }

    fn get_waiting_time(&self, retryable: &impl Retryable) -> Duration {
        self.delay * self.multiplier.pow(retryable.attempt() as u32) as u32
    }
}
