//! response wire schema.
use super::*;
use serde::{Deserialize, Serialize};
// ============== Response Types ==============

#[derive(Debug, Deserialize)]
pub(crate) struct ChatResponse {
    pub(crate) id: String,
    pub(crate) choices: Vec<ChatChoice>,
    pub(crate) usage: Option<ChatUsage>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ChatChoice {
    pub(crate) index: usize,
    pub(crate) message: ChatResponseMessage,
    pub(crate) finish_reason: Option<String>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ChatResponseMessage {
    pub(crate) role: String,
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) tool_calls: Option<Vec<ToolCall>>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ChatUsage {
    pub(crate) prompt_tokens: usize,
    pub(crate) completion_tokens: usize,
    pub(crate) total_tokens: usize,
}
