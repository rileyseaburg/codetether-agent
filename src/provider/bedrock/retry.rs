//! Retry helper for transient Bedrock errors.
//!
//! Bedrock throttles aggressively under load. This module implements
//! exponential backoff + jitter for `ThrottlingException` (HTTP 429) and
//! `ServiceUnavailableException` (HTTP 503). Non-retryable errors (auth,
//! validation, 4xx other than 429) are returned immediately.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::bedrock::retry::{RetryPolicy, should_retry_status};
//!
//! let policy = RetryPolicy::default();
//! assert_eq!(policy.max_attempts, 4);
//! assert!(should_retry_status(429));
//! assert!(should_retry_status(503));
//! assert!(!should_retry_status(400));
//! assert!(!should_retry_status(200));
//! ```

use rand::RngExt;
use std::time::Duration;

/// Exponential backoff config.
#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    /// Total number of attempts (including the first). `1` disables retry.
    pub max_attempts: u32,
    /// Base delay before the first retry (e.g. 500ms).
    pub base_delay: Duration,
    /// Cap on any single sleep interval.
    pub max_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 4,
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(8),
        }
    }
}

impl RetryPolicy {
    /// Compute the sleep for attempt `n` (1-indexed, first *retry* is n=1).
    /// Applies full jitter: `random(0, min(max_delay, base * 2^(n-1)))`.
    pub fn delay_for(&self, n: u32) -> Duration {
        let shift = n.saturating_sub(1).min(10);
        let ceiling = self
            .base_delay
            .saturating_mul(1u32 << shift)
            .min(self.max_delay);
        let millis = rand::rng().random_range(0..=ceiling.as_millis() as u64);
        Duration::from_millis(millis)
    }
}

/// Return true for HTTP status codes worth retrying.
pub fn should_retry_status(status: u16) -> bool {
    matches!(status, 429 | 500 | 502 | 503 | 504)
}
