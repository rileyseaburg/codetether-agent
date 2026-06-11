//! Thread context shared by mapper calls.

use super::ids::{thread_id_for_session, turn_id_for_session};
use super::time::timestamp_ms;

/// Stable identifiers attached to every thread event.
///
/// # Examples
///
/// ```
/// use codetether_agent::session::thread_events::ThreadEventContext;
///
/// let context = ThreadEventContext::new("session-1", "turn-1");
/// assert_eq!(context.thread_id, "session-1");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadEventContext {
    /// Thread identifier used for storage.
    pub thread_id: String,
    /// Session identifier that produced the event.
    pub session_id: String,
    /// Turn identifier shared by events for one prompt.
    pub turn_id: String,
}

impl ThreadEventContext {
    /// Create a context for an explicit session and turn.
    pub fn new(session_id: impl Into<String>, turn_id: impl Into<String>) -> Self {
        let session_id = session_id.into();
        Self {
            thread_id: thread_id_for_session(&session_id),
            session_id,
            turn_id: turn_id.into(),
        }
    }

    /// Create a context with a timestamp-derived turn id.
    pub fn for_session(session_id: impl Into<String>) -> Self {
        let session_id = session_id.into();
        let turn_id = turn_id_for_session(&session_id, timestamp_ms());
        Self::new(session_id, turn_id)
    }
}
