//! Timestamp helpers for thread events.

use std::time::{SystemTime, UNIX_EPOCH};

/// Return the current Unix timestamp in milliseconds.
///
/// # Examples
///
/// ```
/// use codetether_agent::session::thread_events::timestamp_ms;
///
/// assert!(timestamp_ms() > 0);
/// ```
pub fn timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
