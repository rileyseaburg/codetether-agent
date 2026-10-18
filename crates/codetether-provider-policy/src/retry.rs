//! Retryable upstream error classification.

use super::retry_markers as markers;

/// Returns true when an upstream provider error is worth retrying.
/// Apply the established is retryable upstream error policy.
/// # Arguments
/// Receives the provider, request or error inputs indicated by the signature.
/// # Returns
/// The policy decision without invoking providers or mutating session state.
/// # Examples
/// ```
/// assert!(!codetether_provider_policy::is_retryable_upstream_error(&anyhow::anyhow!("retry limit exhausted")));
/// ```
pub fn is_retryable_upstream_error(err: &anyhow::Error) -> bool {
    let msg = err.to_string().to_ascii_lowercase();
    if msg.contains("retry limit exhausted") {
        return false;
    }
    markers::RETRYABLE_NEEDLES
        .iter()
        .any(|needle| msg.contains(needle))
}
