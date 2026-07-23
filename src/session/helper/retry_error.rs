//! Retryable upstream error classification.

#[path = "retry_error/markers.rs"]
mod markers;

/// Returns true when an upstream provider error is worth retrying.
pub fn is_retryable_upstream_error(err: &anyhow::Error) -> bool {
    let msg = err.to_string().to_ascii_lowercase();
    if msg.contains("retry limit exhausted") {
        return false;
    }
    markers::RETRYABLE_NEEDLES
        .iter()
        .any(|needle| msg.contains(needle))
}
