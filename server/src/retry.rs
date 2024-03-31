use std::time::Duration;

use rand::{thread_rng, Rng};
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

pub struct RandomizeDecoratedRetryPolicy {
    // todo allow decorate every RetryPolicy
    decorated: ExponentialRetryPolicy,
    factor: f64,
}

impl RandomizeDecoratedRetryPolicy {
    pub fn new(decorated: ExponentialRetryPolicy, factor: f64) -> Self {
        Self { decorated, factor }
    }
}

impl RetryPolicy for RandomizeDecoratedRetryPolicy {
    fn is_retryable(&self, retryable: &impl Retryable) -> bool {
        self.decorated.is_retryable(retryable)
    }

    fn get_waiting_time(&self, retryable: &impl Retryable) -> Duration {
        let duration = self.decorated.get_waiting_time(retryable);
        let base = duration.as_millis() as f64;
        let diff = base * self.factor;
        let min = base - diff;
        let max = base + diff;

        let randomized_duration = thread_rng().gen_range(min..=max);

        Duration::from_millis(randomized_duration as u64)
    }
}
