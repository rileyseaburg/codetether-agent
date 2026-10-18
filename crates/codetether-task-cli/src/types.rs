use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct WorkspaceSummary {
    pub(crate) id: String,
    pub(crate) path: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct TriggerRequest {
    pub(crate) prompt: String,
    pub(crate) agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) model: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) worker_personality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) notify_email: Option<String>,
    #[serde(default)]
    pub(crate) metadata: serde_json::Map<String, serde_json::Value>,
}

/// Server response to a workspace task trigger.
///
/// # Examples
/// ```
/// let response: codetether_task_cli::TaskTriggerResponse = serde_json::from_str(r#"{"success":true}"#).unwrap();
/// assert!(response.success);
/// ```

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskTriggerResponse {
    /// Whether the server accepted the trigger.
    pub success: bool,
    /// Session identifier when provided.
    pub session_id: Option<String>,
    /// Server status message.
    pub message: Option<String>,
    /// Resolved remote workspace identifier.
    pub workspace_id: Option<String>,
    /// Agent selected by the server.
    pub agent: Option<String>,
    /// Whether the task uses Knative.
    pub knative: Option<bool>,
    /// Server error detail.
    pub error: Option<String>,
    /// Additional routing metadata.
    pub routing: Option<serde_json::Value>,
}
