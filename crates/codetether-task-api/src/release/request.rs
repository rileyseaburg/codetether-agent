//! Worker release wire contract. Provenance fields are retained for compatibility.

use serde::Deserialize;

/// Worker payload accepted with optional compatibility fields.
/// # Examples
/// ```
/// let _request: codetether_task_api::release::ReleaseRequest =
///     serde_json::from_str(r#"{"task_id":"task-1","status":"completed"}"#).unwrap();
/// ```
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ReleaseRequest {
    pub task_id: String,
    pub status: String,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub diagnostics: Option<serde_json::Value>,
}
