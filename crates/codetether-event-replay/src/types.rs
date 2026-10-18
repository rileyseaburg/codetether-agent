//! Audit replay query and archive metadata wire records.
use serde::{Deserialize, Serialize};

// ── Event Stream Replay API ──
// Enables auditors to reconstruct sessions from byte-range offsets.
// This is the key compliance feature for SOC 2, FedRAMP, and ATO processes.

#[derive(Deserialize)]
/// Archive session and optional filtering parameters.
/// # Examples
/// ```
/// let query = codetether_event_replay::ReplayQuery { session_id: "session".into(), start_offset: None, end_offset: None, limit: Some(100), tool_name: None };
/// assert_eq!(query.limit, Some(100));
/// ```
pub struct ReplayQuery {
    /// Session ID to replay
    pub session_id: String,
    /// Optional: starting byte offset (for seeking to specific point)
    pub start_offset: Option<u64>,
    /// Optional: ending byte offset
    pub end_offset: Option<u64>,
    /// Optional: limit number of events to return
    pub limit: Option<usize>,
    /// Optional: filter by tool name
    pub tool_name: Option<String>,
}

#[derive(Serialize)]
/// Byte-offset and size metadata for one archive file.
/// # Examples
/// ```
/// let file = codetether_event_replay::EventFileMeta { filename: "events.jsonl".into(), start_offset: 0, end_offset: 10, size_bytes: 10 };
/// assert_eq!(file.size_bytes, 10);
/// ```
pub struct EventFileMeta {
    pub filename: String,
    pub start_offset: u64,
    pub end_offset: u64,
    pub size_bytes: u64,
}
