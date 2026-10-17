//! message response schema.
use super::*;
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct ChoiceMessage {
    #[allow(dead_code)]
    pub(crate) role: Option<String>,
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) tool_calls: Option<Vec<ToolCall>>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ToolCall {
    pub(crate) id: String,
    pub(crate) function: FunctionCall,
    /// Thought signature for Gemini 3.x models
    #[serde(default)]
    pub(crate) extra_content: Option<ExtraContent>,
}
