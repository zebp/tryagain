use std::time::{Duration, Instant};

/// TODO: Document
pub trait Backoff {
    /// If the backoff implementation should allow for the library to retry the failed function.
    fn should_try_again(&mut self, iterations: usize) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct ExponentialBackoff {
    instant: Instant,
}

impl Backoff for ExponentialBackoff {
    fn should_try_again(&mut self, iterations: usize) -> bool {
        let y = 1.25f32.powi(iterations as i32) - 1.0;
        let duration = Duration::from_millis((y * 100.0) as u64);

        if self.instant.elapsed() >= duration {
            self.instant = Instant::now();
            true
        } else {
            false
        }
    }
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            instant: Instant::now(),
        }
    }
}
