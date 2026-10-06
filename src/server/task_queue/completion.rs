//! Worker completion data stored atomically with terminal task status.

use serde::{Deserialize, Serialize};

/// Receipt from the first accepted terminal transition.
/// Absent fields remain absent on the wire for compatibility with older tasks.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TaskCompletion {
    /// Successful execution result, if supplied by the worker.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// Worker-reported error or nonfatal diagnostic message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Execution session associated with the outcome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Structured execution diagnostics, retained without reinterpretation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<serde_json::Value>,
}
