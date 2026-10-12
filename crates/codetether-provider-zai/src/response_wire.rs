//! Buffered chat-completion response schema.
use crate::ZaiUsage;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub(crate) struct ZaiResponse {
    pub(crate) choices: Vec<ZaiChoice>,
    #[serde(default)]
    pub(crate) usage: Option<ZaiUsage>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ZaiChoice {
    pub(crate) message: ZaiMessage,
    #[serde(default)]
    pub(crate) finish_reason: Option<String>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ZaiMessage {
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) tool_calls: Option<Vec<ZaiToolCall>>,
    #[serde(default)]
    pub(crate) reasoning_content: Option<String>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ZaiToolCall {
    pub(crate) id: String,
    pub(crate) function: ZaiFunction,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ZaiFunction {
    pub(crate) name: String,
    pub(crate) arguments: Value,
}
