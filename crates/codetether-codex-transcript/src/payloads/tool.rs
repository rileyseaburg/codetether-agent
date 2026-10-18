//! Codex tool record payload schemas.
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub(crate) struct CodexFunctionCallPayload {
    pub(crate) name: String,
    pub(crate) arguments: Value,
    #[serde(default)]
    pub(crate) call_id: Option<String>,
    #[serde(default)]
    pub(crate) id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexFunctionCallOutputPayload {
    pub(crate) call_id: String,
    pub(crate) output: Value,
}
