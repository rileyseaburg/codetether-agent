//! Decorrelated-jitter reconnect backoff (Phase 5).
//!
//! Implements the "decorrelated jitter" algorithm: each delay is sampled
//! uniformly from `[base, prev * 3]`, capped at `max`. This spreads reconnect
//! attempts across competing clients far better than fixed or pure-exponential
//! backoff, avoiding the synchronized-retry thundering herd after a server flap.
//! See `docs/transport-first-class-plan.md` Phase 5.

use std::time::Duration;

/// Stateful decorrelated-jitter backoff sampler.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::stream::backoff::Backoff;
/// use std::time::Duration;
///
/// let mut backoff = Backoff::new(Duration::from_millis(100), Duration::from_secs(30));
/// let first = backoff.next_delay();
/// assert!(first >= Duration::from_millis(100));
/// assert!(first <= Duration::from_secs(30));
/// backoff.reset();
/// ```
pub struct Backoff {
    base: Duration,
    max: Duration,
    prev: Duration,
}

impl Backoff {
    /// Create a backoff sampling from `base` up to `max`.
    pub fn new(base: Duration, max: Duration) -> Self {
        Self { base, max, prev: base }
    }

    /// Sample the next delay from `[base, prev * 3]`, capped at `max`.
    pub fn next_delay(&mut self) -> Duration {
        let ceil = (self.prev.saturating_mul(3)).min(self.max).max(self.base);
        let span = ceil.saturating_sub(self.base).as_millis() as u64;
        let jitter = if span == 0 { 0 } else { pseudo_rand() % (span + 1) };
        let delay = self.base + Duration::from_millis(jitter);
        self.prev = delay.min(self.max);
        self.prev
    }

    /// Reset the backoff to its base after a successful connection.
    pub fn reset(&mut self) {
        self.prev = self.base;
    }
}

/// Cheap process-local pseudo-random source (no external rng dependency).
fn pseudo_rand() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(0);
    nanos.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407)
}

#[cfg(test)]
#[path = "backoff_tests.rs"]
mod tests;
