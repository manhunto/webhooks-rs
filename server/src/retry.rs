use std::time::Duration;

use rand::{Rng, thread_rng};

use crate::retry::RetryPolicyConfig::Exponential;

pub trait RetryPolicy {
    fn is_retryable(&self, attempt: usize) -> bool;

    fn get_waiting_time(&self, attempt: usize) -> Duration;
}

struct ExponentialRetryPolicy {
    max_retries: usize,
    multiplier: usize,
    delay: Duration,
}

impl ExponentialRetryPolicy {
    fn new(max_retries: usize, multiplier: usize, delay: Duration) -> Self {
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

struct RandomizeDecoratedRetryPolicy {
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

#[derive(Clone)]
enum RetryPolicyConfig {
    Exponential(ExponentialConfig),
}

#[derive(Clone)]
struct ExponentialConfig {
    multiplier: usize,
    delay: Duration,
}

pub struct RetryPolicyBuilder {
    max_retries: Option<usize>,
    config: Option<RetryPolicyConfig>,
    random_factor: Option<f64>,
}

impl RetryPolicyBuilder {
    pub fn new() -> Self {
        Self {
            max_retries: None,
            config: None,
            random_factor: None,
        }
    }

    pub fn max_retries(&mut self, max_retries: usize) -> &mut Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn exponential(&mut self, multiplier: usize, delay: Duration) -> &mut Self {
        self.config = Some(Exponential(ExponentialConfig { multiplier, delay }));
        self
    }

    pub fn randomize(&mut self, factor: f64) -> &mut Self {
        self.random_factor = Some(factor);
        self
    }

    pub fn build(&self) -> Result<Box<dyn RetryPolicy>, String> {
        if self.max_retries.is_none() {
            return Err(String::from("Max retries should be set"));
        }

        if self.config.is_none() {
            return Err(String::from("Any base policy wasn't chosen"));
        }

        let mut policy: Box<dyn RetryPolicy> = match self.config.clone().unwrap() {
            Exponential(config) => Box::new(ExponentialRetryPolicy::new(
                self.max_retries.unwrap(),
                config.multiplier,
                config.delay,
            )),
        };

        if let Some(factor) = self.random_factor {
            policy = Box::new(RandomizeDecoratedRetryPolicy::new(policy, factor));
        }

        Ok(policy)
    }
}
