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
/// formula `delay = 100(base^iterations - 1)` measured in milliseconds.
#[derive(Debug, Clone, Copy)]
pub struct ExponentialBackoff {
    base: f32,
    instant: Instant,
}

impl ExponentialBackoff {
    /// Creates an [ExponentialBackoff](crate::backoff::ExponentialBackoff) with a base for the exponential
    /// function used to calculate backoff duration.
    ///
    /// Equation: `delay = 100(base^iterations - 1)`
    pub fn with_base(base: f32) -> Self {
        Self {
            base,
            instant: Instant::now(),
        }
    }
}

impl Backoff for ExponentialBackoff {
    fn backoff_period(&mut self, iterations: u32) -> Duration {
        let y = self.base.powi(iterations as i32) - 1.0;
        Duration::from_millis((y * 100.0) as u64)
    }
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            base: 1.25,
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

/// A [Backoff](crate::backoff::Backoff) implementation with a minimum duration
/// that must be reached before a retry attempt can be made.
pub struct MinimumBackoff<T: Backoff> {
    inner: T,
    min_duration: Duration,
}

impl<T: Backoff> MinimumBackoff<T> {
    pub fn new(inner: T, min_duration: Duration) -> Self {
        Self {
            inner,
            min_duration,
        }
    }
}

impl<T: Backoff> Backoff for MinimumBackoff<T> {
    fn backoff_period(&mut self, iterations: u32) -> Duration {
        self.min_duration.max(self.inner.backoff_period(iterations))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_default_exponential() {
        let mut backoff = ExponentialBackoff::default();

        assert_eq!(backoff.backoff_period(0), Duration::from_millis(0));
        assert_eq!(backoff.backoff_period(1), Duration::from_millis(25));
        assert_eq!(backoff.backoff_period(2), Duration::from_millis(56));
        assert_eq!(backoff.backoff_period(3), Duration::from_millis(95));
    }

    #[test]
    fn test_exponential_with_base() {
        let mut backoff = ExponentialBackoff::with_base(10.0);

        assert_eq!(backoff.backoff_period(0), Duration::from_millis(00000));
        assert_eq!(backoff.backoff_period(1), Duration::from_millis(00900));
        assert_eq!(backoff.backoff_period(2), Duration::from_millis(09900));
        assert_eq!(backoff.backoff_period(3), Duration::from_millis(99900));
    }

    #[test]
    fn test_immediate() {
        assert_eq!(ImmediateBackoff.backoff_period(0), Duration::from_millis(0));
    }

    #[test]
    fn test_minimum() {
        let mut backoff = MinimumBackoff::new(ImmediateBackoff, Duration::from_secs(1));
        assert_eq!(backoff.backoff_period(0), Duration::from_secs(1));
    }
}
