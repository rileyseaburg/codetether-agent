//! Jittered exponential backoff matching Codex sampling retries.

use rand::RngExt;
use std::time::Duration;

/// Return exponential `base * multiplier^attempt` with ±10% jitter.
pub(crate) fn jittered(base: Duration, multiplier: u32, attempt: u32) -> Duration {
    if base.is_zero() {
        return base;
    }
    let factor = multiplier.saturating_pow(attempt);
    let nominal = base.checked_mul(factor).unwrap_or(Duration::MAX);
    let jitter = rand::rng().random_range(0.9..1.1);
    Duration::try_from_secs_f64(nominal.as_secs_f64() * jitter).unwrap_or(Duration::MAX)
}
