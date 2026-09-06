use super::ChatTool;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct ChatRequest {
    pub(super) model: String,
    pub(super) messages: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) tools: Option<Vec<ChatTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) stream: Option<bool>,
}
