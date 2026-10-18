//! Codex reasoning record payload schemas.
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub(crate) struct CodexReasoningPayload {
    #[serde(default)]
    pub(crate) summary: Vec<Value>,
    #[serde(default)]
    pub(crate) content: Option<String>,
}
