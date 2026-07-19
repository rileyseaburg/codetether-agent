//! Stream-disconnect error classification.
//!
//! Distinguishes transient mid-stream disconnects (broken pipe, premature EOF,
//! h2/network resets) from permanent provider errors. Only transient disconnects
//! are eligible for same-model session auto-renew.

const DISCONNECT_MARKERS: &[&str] = &[
    "premature eof",
    "unexpected eof",
    "broken pipe",
    "connection reset",
    "connection closed",
    "stream closed",
    "h2 protocol error",
    "h2 error",
    "hyper: connection",
    "network error",
    "transport error",
    "tls handshake",
    "incomplete message",
    "body read",
    "end of stream",
];

const PERMANENT_MARKERS: &[&str] = &[
    "retry limit exhausted",
    "401",
    "403",
    "invalid api key",
    "unauthorized",
    "context length",
    "context_length",
    "maximum context",
    "content filter",
    "content_policy",
];

/// Returns `true` when `error` looks like a transient stream disconnect
/// worth retrying on the same model.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::app::stream_disconnect::is_stream_disconnect;
/// assert!(is_stream_disconnect("premature EOF after 42 bytes"));
/// assert!(!is_stream_disconnect("401 Unauthorized"));
/// ```
pub fn is_stream_disconnect(error: &str) -> bool {
    let lower = error.to_ascii_lowercase();
    if PERMANENT_MARKERS.iter().any(|m| lower.contains(m)) {
        return false;
    }
    DISCONNECT_MARKERS.iter().any(|m| lower.contains(m))
}

#[cfg(test)]
#[path = "stream_disconnect_tests.rs"]
mod tests;
