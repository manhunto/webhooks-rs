use serde::Serialize;

pub trait Retryable<T: Serialize = Self>: Serialize
{
    fn attempt(&self) -> usize;
    fn with_increased_attempt(&self) -> Self;
}
