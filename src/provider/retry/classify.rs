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
/// `true` for 429 (rate-limit) and common upstream gateway / availability
/// failures, `false` otherwise.
pub(super) fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    matches!(
        status.as_u16(),
        429 | 500 | 502 | 503 | 504 | 520 | 522 | 524
    )
}

/// Check whether an error body indicates a *permanent* failure that must
/// not be retried, even when the HTTP status is otherwise retryable (e.g. a
/// 429 whose body is really a billing/subscription problem).
///
/// Providers like Z.AI return HTTP 429 for an expired plan (`code 1309`),
/// which no amount of backoff can fix. Retrying just wastes the full attempt
/// budget before surfacing the real cause, so callers should bail early.
///
/// # Arguments
///
/// * `msg` — The non-success response body or error string.
pub(super) fn is_permanent_message(msg: &str) -> bool {
    let lower = msg.to_lowercase();
    lower.contains("has expired")
        || lower.contains("package has expired")
        || lower.contains("renewing the subscription")
        || lower.contains("renew the subscription")
        || lower.contains("subscription has expired")
        || lower.contains("\"1309\"")
        || lower.contains("code\":1309")
        || lower.contains("insufficient balance")
        || lower.contains("account has been suspended")
}

/// Check whether an error message string indicates a transient failure.
///
/// Catches provider-specific messages like Z.AI's "temporarily overloaded"
/// that may arrive in a non-2xx JSON error body or a network-level error.
///
/// **Important**: This function must only be called on **non-success**
/// response bodies or error strings. Applying it to a 200 OK body risks
/// false positives when legitimate content contains phrases like
/// "operation failed" or "internal server error" in tool results or
/// conversation history.
///
/// # Arguments
///
/// * `msg` — The error message text (from response body or `anyhow::Error`).
pub(super) fn is_retryable_message(msg: &str) -> bool {
    let lower = msg.to_lowercase();
    lower.contains("temporarily overloaded")
        || lower.contains("rate limit")
        || lower.contains("connection reset")
        || lower.contains("connection closed")
        || lower.contains("service unavailable")
        || lower.contains("bad gateway")
        || lower.contains("token_quota_exceeded")
        || lower.contains("too many tokens")
        || lower.contains("limit exceeded")
        || lower.contains("quota exceeded")
        || lower.contains("too many requests")
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
