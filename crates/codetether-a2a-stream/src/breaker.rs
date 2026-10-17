//! Circuit breaker for repeated stream-connection failures (Phase 5).
//!
//! After `threshold` consecutive failures the breaker opens, signalling the
//! lifecycle to stop hammering a server that is hard-down. A single success
//! closes it and resets the count. See `docs/transport-first-class-plan.md`
//! Phase 5.

/// A consecutive-failure circuit breaker.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_stream::breaker::CircuitBreaker;
///
/// let mut breaker = CircuitBreaker::new(3);
/// assert!(!breaker.is_open());
/// breaker.record_failure();
/// breaker.record_failure();
/// assert!(!breaker.is_open());
/// breaker.record_failure();
/// assert!(breaker.is_open());
/// breaker.record_success();
/// assert!(!breaker.is_open());
/// ```
pub struct CircuitBreaker {
    threshold: u32,
    consecutive_failures: u32,
}

impl CircuitBreaker {
    /// Create a breaker that opens after `threshold` consecutive failures.
    pub fn new(threshold: u32) -> Self {
        Self {
            threshold,
            consecutive_failures: 0,
        }
    }

    /// Record a connection failure, incrementing the consecutive count.
    pub fn record_failure(&mut self) {
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
    }

    /// Record a connection success, closing the breaker.
    pub fn record_success(&mut self) {
        self.consecutive_failures = 0;
    }

    /// Whether the breaker is open (failures reached the threshold).
    pub fn is_open(&self) -> bool {
        self.consecutive_failures >= self.threshold
    }

    /// Current consecutive-failure count (for observability).
    pub fn failure_count(&self) -> u32 {
        self.consecutive_failures
    }
}
#[cfg(test)]
#[path = "breaker_tests.rs"]
mod tests;
