//! Exponential retry-delay calculation.

use tokio::time::Duration;

#[cfg(test)]
#[path = "backoff_tests.rs"]
mod tests;

/// Calculates a capped exponential retry delay.
///
/// # Arguments
///
/// * `attempt` — Zero-based retry attempt.
/// * `initial_ms` — Delay used for the first retry.
/// * `maximum_ms` — Upper bound for the returned delay.
/// * `multiplier` — Exponential growth factor.
///
/// # Returns
///
/// A duration no greater than `maximum_ms`.
///
/// # Examples
///
/// ```rust
/// # use std::time::Duration;
/// # fn delay(attempt: u32, initial: u64, max: u64, multiplier: f64) -> Duration {
/// # let millis = (initial as f64 * multiplier.powi(attempt as i32)).min(max as f64);
/// # Duration::from_millis(millis as u64) }
/// assert_eq!(delay(2, 100, 1_000, 2.0), Duration::from_millis(400));
/// ```
pub(super) fn calculate(
    attempt: u32,
    initial_ms: u64,
    maximum_ms: u64,
    multiplier: f64,
) -> Duration {
    let delay = (initial_ms as f64 * multiplier.powi(attempt as i32)).min(maximum_ms as f64);
    Duration::from_millis(delay as u64)
}
