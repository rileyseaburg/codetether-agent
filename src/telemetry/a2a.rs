//! A2A (Agent-to-Agent) protocol message telemetry records.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// One A2A message exchange between the worker and a remote agent.
///
/// `blocking` indicates whether the caller awaited a response. `output` and
/// `error` are mutually exclusive based on `success`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::A2AMessageRecord;
/// use chrono::Utc;
///
/// let r = A2AMessageRecord {
///     tool_name: "delegate".into(),
///     task_id: "task-1".into(),
///     blocking: true,
///     prompt: "hello".into(),
///     duration_ms: 120,
///     success: true,
///     output: Some("hi".into()),
///     error: None,
///     timestamp: Utc::now(),
/// };
/// assert!(r.success);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2AMessageRecord {
    /// Tool name that produced the message (e.g. `"delegate"`).
    pub tool_name: String,
    /// A2A task id the message belongs to.
    pub task_id: String,
    /// `true` if the caller awaited a synchronous response.
    pub blocking: bool,
    /// Prompt / request body sent to the remote agent.
    pub prompt: String,
    /// Round-trip duration in milliseconds.
    pub duration_ms: u64,
    /// `true` iff the remote agent returned a non-error response.
    pub success: bool,
    /// Response body, when `success` is `true`.
    pub output: Option<String>,
    /// Error message, when `success` is `false`.
    pub error: Option<String>,
    /// When the exchange completed.
    pub timestamp: DateTime<Utc>,
}
