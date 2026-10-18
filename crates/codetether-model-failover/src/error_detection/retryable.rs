//! Generic provider-error retry classification.

use super::markers::{MARKERS, NON_RETRYABLE_STATUS_CODES, TRANSIENT_STATUS_CODES};

/// Classifies provider errors as retryable (rate limit, timeout, 5xx) vs permanent.
///
/// # Arguments
/// * `err` — err input.
///
/// # Returns
/// Whether the documented policy condition matches.
///
/// # Examples
/// ```
/// assert!(codetether_model_failover::is_retryable_provider_error("timeout"));
/// ```
pub fn is_retryable_provider_error(err: &str) -> bool {
    let normalized = err.to_ascii_lowercase();

    if normalized.contains("retry limit exhausted")
        || NON_RETRYABLE_STATUS_CODES
            .iter()
            .any(|c| normalized.contains(c))
    {
        return false;
    }
    MARKERS.iter().any(|m| normalized.contains(m))
        || TRANSIENT_STATUS_CODES
            .iter()
            .any(|c| normalized.contains(c))
}
