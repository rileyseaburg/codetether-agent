//! Deserialization types for the `session_task` tool.

use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Params {
    pub action: String,
    #[serde(default)]
    pub objective: Option<String>,
    #[serde(default)]
    pub success_criteria: Option<Vec<String>>,
    #[serde(default)]
    pub forbidden: Option<Vec<String>>,
    #[serde(default)]
    pub progress_note: Option<String>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    /// Injected by `enrich_tool_input_with_runtime_context`.
    #[serde(default, rename = "__ct_session_id")]
    pub ct_session_id: Option<String>,
}
