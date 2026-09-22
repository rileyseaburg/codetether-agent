//! Wire types returned by GitHub Copilot chat completions.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(in crate::provider) struct CopilotResponse {
    pub(super) choices: Vec<CopilotChoice>,
    #[serde(default)]
    pub(super) usage: Option<CopilotUsage>,
}

#[derive(Debug, Deserialize)]
pub(super) struct CopilotChoice {
    pub(super) message: CopilotMessage,
    #[serde(default)]
    pub(super) finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct CopilotMessage {
    #[serde(default)]
    pub(super) content: Option<String>,
    #[serde(default)]
    pub(super) tool_calls: Option<Vec<CopilotToolCall>>,
}

#[derive(Debug, Deserialize)]
pub(super) struct CopilotToolCall {
    pub(super) id: String,
    pub(super) function: CopilotFunction,
}

#[derive(Debug, Deserialize)]
pub(super) struct CopilotFunction {
    pub(super) name: String,
    #[serde(default)]
    pub(super) arguments: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct CopilotUsage {
    #[serde(default)]
    pub(super) prompt_tokens: usize,
    #[serde(default)]
    pub(super) completion_tokens: usize,
    #[serde(default)]
    pub(super) total_tokens: usize,
}
