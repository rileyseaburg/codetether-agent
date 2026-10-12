//! Jittered exponential backoff for desynchronized retries.

use rand::RngExt;
use std::time::Duration;

/// Return exponential `base * multiplier^attempt` with ±10% jitter.
///
/// # Arguments
///
/// * `base` — Initial delay; zero remains zero.
/// * `multiplier` — Exponential growth factor.
/// * `attempt` — Zero-based retry counter.
///
/// # Returns
///
/// Jittered delay, saturating to [`Duration::MAX`] on overflow.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::retry::timing::jittered;
/// use std::time::Duration;
/// assert_eq!(jittered(Duration::ZERO, 2, 8), Duration::ZERO);
/// ```
pub fn jittered(base: Duration, multiplier: u32, attempt: u32) -> Duration {
    if base.is_zero() {
        return base;
    }
    let factor = multiplier.saturating_pow(attempt);
    let nominal = base.checked_mul(factor).unwrap_or(Duration::MAX);
    let jitter = rand::rng().random_range(0.9..1.1);
    Duration::try_from_secs_f64(nominal.as_secs_f64() * jitter).unwrap_or(Duration::MAX)
}
