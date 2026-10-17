//! Assistant content wire schema.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct MoonshotMessage {
    #[allow(dead_code)]
    pub(crate) role: String,
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) tool_calls: Option<Vec<MoonshotToolCall>>,
    // Kimi K2.5 reasoning
    #[serde(default)]
    pub(crate) reasoning_content: Option<String>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct MoonshotToolCall {
    pub(crate) id: String,
    #[serde(rename = "type")]
    pub(crate) call_type: String,
    pub(crate) function: MoonshotFunction,
}
#[derive(Debug, Deserialize)]
pub(crate) struct MoonshotFunction {
    pub(crate) name: String,
    pub(crate) arguments: String,
}
