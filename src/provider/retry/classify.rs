use std::time::Duration;

/// Check whether an HTTP status code is a transient server error.
///
/// Used by [`super::send_with_retry`] and [`super::send_response_with_retry`]
/// to decide whether to retry a failed provider request.
///
/// # Arguments
///
/// * `status` — The HTTP status code from the provider response.
///
/// # Returns
///
/// `true` for 429 (rate-limit) and common 5xx codes, `false` otherwise.
pub(super) fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    matches!(status.as_u16(), 429 | 502 | 503 | 504 | 520 | 522 | 524)
}

/// Check whether an error message string indicates a transient failure.
///
/// Catches provider-specific messages like Z.AI's "temporarily overloaded"
/// that may arrive in a 200-status JSON error body or a network-level error.
///
/// # Arguments
///
/// * `msg` — The error message text (from response body or `anyhow::Error`).
pub(super) fn is_retryable_message(msg: &str) -> bool {
    let lower = msg.to_lowercase();
    lower.contains("temporarily overloaded")
        || lower.contains("rate limit")
        || lower.contains("timed out")
        || lower.contains("connection reset")
        || lower.contains("connection closed")
}

/// Compute exponential backoff delay for a given retry attempt.
///
/// Produces 2s, 4s, 8s, 16s, then caps at 30s for all subsequent attempts.
///
/// # Arguments
///
/// * `attempt` — 1-based attempt counter.
pub(super) fn backoff_delay(attempt: u32) -> Duration {
    Duration::from_secs(2u64.saturating_pow(attempt).min(30))
}
