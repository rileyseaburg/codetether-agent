//! OpenRouter's OpenAI-compatible streaming response types.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct Response {
    #[serde(default)]
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub usage: Option<super::super::OpenRouterUsage>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Choice {
    #[serde(default)]
    pub delta: Delta,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub(super) struct Delta {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub reasoning: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ToolCall {
    #[serde(default)]
    pub index: usize,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub function: Option<Function>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Function {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub arguments: Option<String>,
}
