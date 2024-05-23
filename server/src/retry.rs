use std::time::Duration;

use rand::{thread_rng, Rng};

use crate::retry::RetryPolicyConfig::{Constant, Exponential};

type ShouldRetryPolicyType = dyn ShouldRetryPolicy + Sync + Send;
type DelayRetryPolicyType = dyn RetryPolicy + Sync + Send;
type RandomGeneratorType = dyn RandomGenerator + Sync + Send;

pub struct Retry {
    should_retry_policy: Box<ShouldRetryPolicyType>,
    delay_retry_policy: Box<DelayRetryPolicyType>,
}

impl Retry {
    fn new(
        should_retry_policy: Box<ShouldRetryPolicyType>,
        delay_retry_policy: Box<DelayRetryPolicyType>,
    ) -> Self {
        Self {
            should_retry_policy,
            delay_retry_policy,
        }
    }

    pub fn is_retryable(&self, attempt: usize) -> bool {
        self.should_retry_policy.is_retryable(attempt)
    }

    pub fn get_waiting_time(&self, attempt: usize) -> Duration {
        self.delay_retry_policy.get_waiting_time(attempt)
    }
}

pub trait ShouldRetryPolicy {
    fn is_retryable(&self, attempt: usize) -> bool;
}

struct MaxAttemptsShouldRetryPolicy {
    max_retries: usize,
}

impl MaxAttemptsShouldRetryPolicy {
    fn new(max_retries: usize) -> Self {
        Self { max_retries }
    }
}

impl ShouldRetryPolicy for MaxAttemptsShouldRetryPolicy {
    fn is_retryable(&self, attempt: usize) -> bool {
        attempt < self.max_retries
    }
}

// todo extract Attempt, validation at least 1
trait RetryPolicy {
    fn get_waiting_time(&self, attempt: usize) -> Duration;
}

struct ExponentialRetryPolicy {
    delay: Duration,
    multiplier: usize,
}

impl ExponentialRetryPolicy {
    fn new(config: ExponentialConfig) -> Self {
        Self {
            delay: config.delay,
            multiplier: config.multiplier,
        }
    }
}

impl RetryPolicy for ExponentialRetryPolicy {
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
    random_generator: Box<RandomGeneratorType>,
    decorated: Box<DelayRetryPolicyType>,
    factor: f64,
}

impl RandomizeDecoratedRetryPolicy {
    pub fn new(
        random_generator: Box<RandomGeneratorType>,
        decorated: Box<DelayRetryPolicyType>,
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
    delay: Duration,
}

impl ConstantRetryPolicy {
    fn new(config: ConstantConfig) -> Self {
        Self {
            delay: config.delay,
        }
    }
}

impl RetryPolicy for ConstantRetryPolicy {
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

impl ExponentialConfig {
    fn new(multiplier: usize, delay: Duration) -> Self {
        Self { multiplier, delay }
    }
}

#[derive(Clone)]
struct ConstantConfig {
    delay: Duration,
}

impl ConstantConfig {
    fn new(delay: Duration) -> Self {
        Self { delay }
    }
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
        self.config = Some(Exponential(ExponentialConfig::new(multiplier, delay)));
        self
    }

    pub fn constant(&mut self, delay: Duration) -> &mut Self {
        self.config = Some(Constant(ConstantConfig::new(delay)));
        self
    }

    pub fn randomize(&mut self, factor: f64) -> &mut Self {
        self.random_factor = Some(factor);
        self
    }

    pub fn build(&self) -> Result<Retry, String> {
        if self.max_retries.is_none() {
            return Err(String::from("Max retries should be set"));
        }

        if self.config.is_none() {
            return Err(String::from("Any base policy wasn't chosen"));
        }

        let mut delay_policy: Box<DelayRetryPolicyType> = match self.config.clone().unwrap() {
            Exponential(config) => Box::new(ExponentialRetryPolicy::new(config)),
            Constant(config) => Box::new(ConstantRetryPolicy::new(config)),
        };

        if let Some(factor) = self.random_factor {
            delay_policy = Box::new(RandomizeDecoratedRetryPolicy::new(
                Box::new(RandCrateRandomGenerator::new()),
                delay_policy,
                factor,
            ));
        }

        let should_retry = Box::new(MaxAttemptsShouldRetryPolicy::new(self.max_retries.unwrap()));

        Ok(Retry::new(should_retry, delay_policy))
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
        ConstantConfig, ConstantRetryPolicy, ExponentialConfig, ExponentialRetryPolicy,
        RandomGenerator, RandomGeneratorType, RandomizeDecoratedRetryPolicy, RetryPolicy,
    };

    // todo write tests for builder with the same cases
    #[test_case(1, 2, 1, 2)]
    #[test_case(1, 2, 2, 4)]
    #[test_case(1, 2, 3, 8)]
    #[test_case(1, 2, 4, 16)]
    #[test_case(2, 3, 1, 6)]
    #[test_case(2, 3, 3, 54)]
    fn exponential_retry_policy(
        delay_in_secs: u64,
        multiplier: usize,
        attempt: usize,
        result: u64,
    ) {
        let config = ExponentialConfig::new(multiplier, Duration::from_secs(delay_in_secs));
        let sut = ExponentialRetryPolicy::new(config);

        assert_eq!(Duration::from_secs(result), sut.get_waiting_time(attempt));
    }

    // todo write tests for builder with the same cases
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
        random_generator: Box<RandomGeneratorType>,
        factor: f64,
    ) -> RandomizeDecoratedRetryPolicy {
        let config = ConstantConfig::new(Duration::from_millis(delay));
        let constant = Box::new(ConstantRetryPolicy::new(config));

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
