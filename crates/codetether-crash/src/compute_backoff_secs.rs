//! Crash reporting compute backoff secs.
use super::*;
pub(super) fn compute_backoff_secs(consecutive_failures: u32) -> u64 {
    // Exponential backoff: 5m, 10m, 20m, ... capped at 24h
    // Clamp exponent so we don't overflow.
    let exp = consecutive_failures.saturating_sub(1).min(10);
    let factor = 1u64.checked_shl(exp).unwrap_or(u64::MAX);
    let backoff = MIN_UPLOAD_BACKOFF_SECS.saturating_mul(factor);
    backoff.clamp(MIN_UPLOAD_BACKOFF_SECS, MAX_UPLOAD_BACKOFF_SECS)
}
