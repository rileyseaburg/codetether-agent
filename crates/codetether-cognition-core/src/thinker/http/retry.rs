//! Transient-failure classification for HTTP thinker requests.

/// Returns true for HTTP status codes that are worth retrying.
///
/// # Arguments
/// * `status` - Numeric HTTP status.
/// # Returns
/// Whether the existing retry policy allows another attempt.
/// # Examples
/// ```
/// use codetether_cognition_core::thinker::http::retry::is_transient_http_error;
/// assert!(is_transient_http_error(429));
/// assert!(!is_transient_http_error(401));
/// ```
pub fn is_transient_http_error(status: u16) -> bool {
    matches!(status, 429 | 502 | 503 | 504)
}

/// Returns true for reqwest errors worth retrying (timeouts, connection resets).
///
/// # Arguments
/// * `e` - Reqwest error returned by a request attempt.
/// # Returns
/// Whether the error is a timeout, connection or request failure.
/// # Examples
/// ```
/// # fn classify(error: &reqwest::Error) -> bool {
/// codetether_cognition_core::thinker::http::retry::is_transient_reqwest_error(error)
/// # }
/// ```
pub fn is_transient_reqwest_error(e: &reqwest::Error) -> bool {
    e.is_timeout() || e.is_connect() || e.is_request()
}

#[cfg(test)]
#[path = "retry_tests.rs"]
mod tests;
