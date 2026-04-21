//! A single tool invocation record with timing, outcome, and file changes.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::FileChange;

/// Telemetry record for one tool invocation.
///
/// Build one with [`ToolExecution::start`] at the top of your tool's
/// `execute()` impl, then finalize it with [`ToolExecution::complete`] /
/// [`ToolExecution::fail`] before returning.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::ToolExecution;
/// use serde_json::json;
///
/// let mut exec = ToolExecution::start("read", json!({"path": "foo.rs"}));
/// assert!(!exec.success);
/// exec.complete(true, 42);
/// assert!(exec.success);
/// assert_eq!(exec.duration_ms, 42);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecution {
    /// Unique id for cross-referencing with the audit log.
    pub id: String,
    /// Registered tool name (e.g. `"bash"`, `"read"`).
    pub tool_name: String,
    /// When execution *started*.
    pub timestamp: DateTime<Utc>,
    /// Wall-clock duration in milliseconds.
    pub duration_ms: u64,
    /// `true` iff the tool returned Ok.
    pub success: bool,
    /// Error message when `success` is `false`.
    pub error: Option<String>,
    /// Tokens attributed to this invocation, when known.
    pub tokens_used: Option<u64>,
    /// Owning session's UUID, when available.
    pub session_id: Option<String>,
    /// Raw tool input as JSON.
    pub input: Option<serde_json::Value>,
    /// Files the tool touched during this invocation.
    #[serde(default)]
    pub file_changes: Vec<FileChange>,
}

mod execution_methods;
