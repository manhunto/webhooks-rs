use std::time::Duration;

use rand::{thread_rng, Rng};

use crate::retry::RetryPolicyConfig::{Constant, Exponential};

pub trait RetryPolicy {
    fn is_retryable(&self, attempt: usize) -> bool; // todo extract should retry trait or something

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

trait RandomGenerator {
    fn random(&self, min: f64, max: f64) -> f64;
}

struct RandCrateRandomGenerator {}

impl RandCrateRandomGenerator {
    fn new() -> Self {
        Self {}
    }
}

impl RandomGenerator for RandCrateRandomGenerator {
    fn random(&self, min: f64, max: f64) -> f64 {
        thread_rng().gen_range(min..=max)
    }
}

struct RandomizeDecoratedRetryPolicy {
    random_generator: Box<dyn RandomGenerator>,
    decorated: Box<dyn RetryPolicy>,
    factor: f64,
}

impl RandomizeDecoratedRetryPolicy {
    pub fn new(
        random_generator: Box<dyn RandomGenerator>,
        decorated: Box<dyn RetryPolicy>,
        factor: f64,
    ) -> Self {
        Self {
            random_generator,
            decorated,
            factor,
        }
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

        let randomized_duration = self.random_generator.random(min, max);

        Duration::from_millis(randomized_duration as u64)
    }
}

struct ConstantRetryPolicy {
    max_retries: usize,
    delay: Duration,
}

impl ConstantRetryPolicy {
    fn new(max_retries: usize, delay: Duration) -> Self {
        Self { max_retries, delay }
    }
}

impl RetryPolicy for ConstantRetryPolicy {
    fn is_retryable(&self, attempt: usize) -> bool {
        attempt < self.max_retries
    }

    fn get_waiting_time(&self, _attempt: usize) -> Duration {
        self.delay
    }
}

#[derive(Clone)]
enum RetryPolicyConfig {
    Exponential(ExponentialConfig),
    Constant(ConstantConfig),
}

#[derive(Clone)]
struct ExponentialConfig {
    multiplier: usize,
    delay: Duration,
}

#[derive(Clone)]
struct ConstantConfig {
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

    pub fn constant(&mut self, delay: Duration) -> &mut Self {
        self.config = Some(Constant(ConstantConfig { delay }));
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
            Constant(config) => Box::new(ConstantRetryPolicy::new(
                self.max_retries.unwrap(),
                config.delay,
            )),
        };

        if let Some(factor) = self.random_factor {
            policy = Box::new(RandomizeDecoratedRetryPolicy::new(
                Box::new(RandCrateRandomGenerator::new()),
                policy,
                factor,
            ));
        }

        Ok(policy)
    }
}

impl Default for RetryPolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use test_case::test_case;

    use crate::retry::{
        ConstantRetryPolicy, RandomGenerator, RandomizeDecoratedRetryPolicy, RetryPolicy,
    };

    #[test_case(5000, 0.5, 2500, 7500; "5 sec, 0.5 factor")]
    #[test_case(5000, 0.25, 3750, 6250; "5 sec, 0.25 factor")]
    #[test_case(3000, 0.1, 2700, 3300; "3 sec, 0.1 factor")]
    fn randomize_decorated_retry_policy(delay: u64, factor: f64, min: u64, max: u64) {
        let sut =
            build_randomize_decorated_retry_policy(delay, Box::new(MinRandomGenerator {}), factor);

        let min_delay = sut.get_waiting_time(1);
        assert_eq!(Duration::from_millis(min), min_delay);

        let sut =
            build_randomize_decorated_retry_policy(delay, Box::new(MaxRandomGenerator {}), factor);

        let max_delay = sut.get_waiting_time(1);
        assert_eq!(Duration::from_millis(max), max_delay);
    }

    fn build_randomize_decorated_retry_policy(
        delay: u64,
        random_generator: Box<dyn RandomGenerator>,
        factor: f64,
    ) -> RandomizeDecoratedRetryPolicy {
        let constant = Box::new(ConstantRetryPolicy::new(1, Duration::from_millis(delay)));

        RandomizeDecoratedRetryPolicy::new(random_generator, constant, factor)
    }

    struct MinRandomGenerator {}

    impl RandomGenerator for MinRandomGenerator {
        fn random(&self, min: f64, _max: f64) -> f64 {
            min
        }
    }

    struct MaxRandomGenerator {}

    impl RandomGenerator for MaxRandomGenerator {
        fn random(&self, _min: f64, max: f64) -> f64 {
            max
        }
    }
}
