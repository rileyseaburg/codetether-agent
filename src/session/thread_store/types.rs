//! Serializable thread event records.

use serde::{Deserialize, Serialize};

/// One append-only event in a conversation thread.
///
/// # Examples
///
/// ```
/// use codetether_agent::session::thread_store::ThreadEvent;
///
/// let event = ThreadEvent {
///     event_id: "event-1".into(),
///     thread_id: "thread-1".into(),
///     session_id: "session-1".into(),
///     turn_id: "turn-1".into(),
///     kind: "turn.started".into(),
///     timestamp_ms: 1_700_000_000_000,
///     payload: serde_json::json!({ "role": "user" }),
/// };
///
/// assert_eq!(event.thread_id, "thread-1");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadEvent {
    /// Stable unique identifier for this event.
    pub event_id: String,
    /// Thread identifier used to choose the backing JSONL file.
    pub thread_id: String,
    /// Session identifier that produced the event.
    #[serde(default)]
    pub session_id: String,
    /// Turn identifier grouping events within the thread.
    pub turn_id: String,
    /// Application-defined event kind, for example `turn.started`.
    pub kind: String,
    /// Event timestamp in Unix epoch milliseconds.
    pub timestamp_ms: u64,
    /// Structured event payload.
    pub payload: serde_json::Value,
}
