use std::time::Duration;

use rand::{Rng, thread_rng};

pub trait RetryPolicy {
    fn is_retryable(&self, attempt: usize) -> bool;

    fn get_waiting_time(&self, attempt: usize) -> Duration;
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
    fn is_retryable(&self, attempt: usize) -> bool {
        attempt < self.max_retries
    }

    fn get_waiting_time(&self, attempt: usize) -> Duration {
        self.delay * self.multiplier.pow(attempt as u32) as u32
    }
}

pub struct RandomizeDecoratedRetryPolicy {
    decorated: Box<dyn RetryPolicy>,
    factor: f64,
}

impl RandomizeDecoratedRetryPolicy {
    pub fn new(decorated: Box<dyn RetryPolicy>, factor: f64) -> Self {
        Self { decorated, factor }
    }
}

impl RetryPolicy for RandomizeDecoratedRetryPolicy {
    fn is_retryable(&self, attempt: usize) -> bool {
        self.decorated.is_retryable(attempt)
    }

    fn get_waiting_time(&self, attempt: usize) -> Duration {
        let base = self.decorated.get_waiting_time(attempt).as_millis() as f64;
        let diff = base * self.factor;

        let min = base - diff;
        let max = base + diff;

        let randomized_duration = thread_rng().gen_range(min..=max); // todo extract randomizer to unit test edge cases

        Duration::from_millis(randomized_duration as u64)
    }
}
