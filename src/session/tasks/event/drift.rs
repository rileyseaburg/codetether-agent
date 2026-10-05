//! Serialized alignment-warning counters; JSON shape remains unchanged.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Counters observed when session alignment needs attention.
///
/// # Examples
/// ```
/// use codetether_agent::session::tasks::TaskEvent;
/// let event: TaskEvent = serde_json::from_value(serde_json::json!({
///     "kind": "drift_detected", "at": "2026-01-01T00:00:00Z",
///     "tool_calls_since_reaffirm": 4, "errors_since_reaffirm": 1
/// })).unwrap();
/// assert!(matches!(event, TaskEvent::DriftDetected(_)));
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DriftDetected {
    /// Time at which drift was detected.
    pub at: DateTime<Utc>,
    /// Tool calls made since the latest goal reaffirmation.
    pub tool_calls_since_reaffirm: u32,
    /// Errors observed since the latest goal reaffirmation.
    pub errors_since_reaffirm: u32,
}
