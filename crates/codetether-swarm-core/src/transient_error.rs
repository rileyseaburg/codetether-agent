//! Classify textual errors for retry eligibility.

/// Determine whether an error message represents a transient failure that
/// is safe to retry (network issues, rate limiting, timeouts).
///
/// Returns `true` if the error is likely transient and a retry may succeed.
///
/// # Arguments
/// * `error` - Error message to inspect.
/// # Returns
/// Whether the original retryable-error patterns match.
/// # Examples
/// ```
/// assert!(codetether_swarm_core::is_transient_error("connection reset"));
/// assert!(!codetether_swarm_core::is_transient_error("invalid input"));
/// ```
pub fn is_transient_error(error: &str) -> bool {
    let lower = error.to_ascii_lowercase();
    // Rate limiting
    if lower.contains("rate limit")
        || lower.contains("ratelimit")
        || lower.contains("too many requests")
        || lower.contains("429")
    {
        return true;
    }
    // Timeout / connection issues
    if lower.contains("timeout")
        || lower.contains("timed out")
        || lower.contains("connection reset")
        || lower.contains("connection refused")
        || lower.contains("broken pipe")
        || lower.contains("network")
        || lower.contains("socket")
        || lower.contains("io error")
    {
        return true;
    }
    // Transient server errors
    if lower.contains("503")
        || lower.contains("502")
        || lower.contains("504")
        || lower.contains("service unavailable")
        || lower.contains("bad gateway")
        || lower.contains("gateway timeout")
        || lower.contains("overloaded")
        || lower.contains("temporarily unavailable")
    {
        return true;
    }
    false
}
