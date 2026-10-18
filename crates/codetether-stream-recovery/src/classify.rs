//! Classify errors for same-model stream reconnects.
use crate::markers::{DISCONNECT_MARKERS, PERMANENT_MARKERS};

/// Returns `true` when `error` looks like a transient stream disconnect
/// worth retrying on the same model.
///
/// # Examples
///
/// ```rust
/// use codetether_stream_recovery::is_stream_disconnect;
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
