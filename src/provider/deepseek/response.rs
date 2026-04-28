//! Response deserialization types for DeepSeek API.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct DsResponse {
    #[allow(dead_code)]
    pub id: String,
    pub model: String,
    pub choices: Vec<DsChoice>,
    #[serde(default)]
    pub usage: Option<DsUsage>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DsChoice {
    pub message: DsMessage,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DsMessage {
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub reasoning_content: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<DsToolCall>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DsToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: DsFunction,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DsFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DsUsage {
    #[serde(default)]
    pub prompt_tokens: usize,
    #[serde(default)]
    pub completion_tokens: usize,
    #[serde(default)]
    pub total_tokens: usize,
}
