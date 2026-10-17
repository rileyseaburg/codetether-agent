//! Response message and tool-call schema.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct OpenRouterMessage {
    pub(crate) role: String,
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) tool_calls: Option<Vec<OpenRouterToolCall>>,
    // Extended fields from thinking models like Kimi K2.5
    #[serde(default)]
    pub(crate) reasoning: Option<String>,
    #[serde(default)]
    pub(crate) reasoning_details: Option<Vec<Value>>,
    #[serde(default)]
    pub(crate) refusal: Option<String>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct OpenRouterToolCall {
    pub(crate) id: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    pub(crate) call_type: String,
    pub(crate) function: OpenRouterFunction,
    #[serde(default)]
    #[allow(dead_code)]
    pub(crate) index: Option<usize>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct OpenRouterFunction {
    pub(crate) name: String,
    pub(crate) arguments: String,
}
