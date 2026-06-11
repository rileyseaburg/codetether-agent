//! Stable identifier helpers for thread protocol records.

/// Return the thread id used for a session-backed thread.
///
/// # Examples
///
/// ```
/// use codetether_agent::session::thread_events::thread_id_for_session;
///
/// assert_eq!(thread_id_for_session("s1"), "s1");
/// ```
pub fn thread_id_for_session(session_id: &str) -> String {
    session_id.to_string()
}

/// Build a turn id from a session id and timestamp.
///
/// # Examples
///
/// ```
/// use codetether_agent::session::thread_events::turn_id_for_session;
///
/// assert_eq!(turn_id_for_session("s1", 42), "turn-s1-42");
/// ```
pub fn turn_id_for_session(session_id: &str, timestamp_ms: u64) -> String {
    format!("turn-{session_id}-{timestamp_ms}")
}

pub(super) fn event_id(turn_id: &str, seq: u64) -> String {
    format!("{turn_id}-event-{seq:06}")
}

pub(super) fn item_id(turn_id: &str, seq: u64) -> String {
    format!("{turn_id}-item-{seq:06}")
}
