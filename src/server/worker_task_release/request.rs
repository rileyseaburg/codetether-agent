//! Worker release wire contract. Provenance fields are retained for compatibility.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct ReleaseRequest {
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
