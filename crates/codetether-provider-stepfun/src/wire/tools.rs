//! tools wire schema.
use super::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize)]
pub(crate) struct ChatTool {
    pub(crate) r#type: String,
    pub(crate) function: ChatFunction,
}
#[derive(Debug, Serialize)]
pub(crate) struct ChatFunction {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) parameters: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ToolCall {
    pub(crate) id: String,
    pub(crate) r#type: String,
    pub(crate) function: ToolCallFunction,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ToolCallFunction {
    pub(crate) name: String,
    pub(crate) arguments: String,
}
