//! Transient-failure classification for HTTP thinker requests.

/// Returns true for HTTP status codes that are worth retrying.
pub(super) fn is_transient_http_error(status: u16) -> bool {
    matches!(status, 429 | 502 | 503 | 504)
}

/// Returns true for reqwest errors worth retrying (timeouts, connection resets).
pub(super) fn is_transient_reqwest_error(e: &reqwest::Error) -> bool {
    e.is_timeout() || e.is_connect() || e.is_request()
}

#[cfg(test)]
#[path = "retry_tests.rs"]
mod tests;
