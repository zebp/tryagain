use std::time::{Duration, Instant};

/// The implementation of the algorithm used to time when failures should he
/// retried.
pub trait Backoff {
    /// If the backoff implementation should allow for the library to retry the failed function.
    fn backoff_period(&mut self, iterations: u32) -> Duration;
}

/// A [Backoff](crate::backoff::Backoff) implementation that exponentially
/// increases the delay between attempts.
///
/// # Details
/// Currently [ExponentialBackoff](crate::backoff::ExponentialBackoff) uses the
/// formula `delay = 100(1.125^iterations - 1)` measured in milliseconds.
#[derive(Debug, Clone, Copy)]
pub struct ExponentialBackoff {
    instant: Instant,
}

impl Backoff for ExponentialBackoff {
    fn backoff_period(&mut self, iterations: u32) -> Duration {
        let y = 1.25f32.powi(iterations as i32) - 1.0;
        Duration::from_millis((y * 100.0) as u64)
    }
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            instant: Instant::now(),
        }
    }
}

/// A [Backoff](crate::backoff::Backoff) implementation that doesn't have
/// any delay and retries immediately.
pub struct ImmediateBackoff;

impl Backoff for ImmediateBackoff {
    fn backoff_period(&mut self, _iterations: u32) -> Duration {
        Duration::from_secs(0)
    }
}
