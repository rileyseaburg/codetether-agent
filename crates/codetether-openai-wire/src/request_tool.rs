//! OpenAI-compatible request tool wire contracts.
use crate::defaults::{default_function_type, default_json_object};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// OpenAI-compatible OpenAiRequestTool wire representation.
/// # Examples
/// ```
/// let value = serde_json::from_str::<codetether_openai_wire::OpenAiRequestTool>(r#"{"function":{"name":"read"}}"#).unwrap();
/// let _ = value;
/// ```
pub struct OpenAiRequestTool {
    #[serde(rename = "type", default = "default_function_type")]
    /// kind wire field.
    pub kind: String,
    /// function wire field.
    pub function: OpenAiRequestToolDefinition,
}

#[derive(Debug, Deserialize)]
/// OpenAI-compatible OpenAiRequestToolDefinition wire representation.
/// # Examples
/// ```
/// let value = serde_json::from_str::<codetether_openai_wire::OpenAiRequestToolDefinition>(r#"{"name":"read"}"#).unwrap();
/// let _ = value;
/// ```
pub struct OpenAiRequestToolDefinition {
    /// name wire field.
    pub name: String,
    #[serde(default)]
    /// description wire field.
    pub description: String,
    #[serde(default = "default_json_object")]
    /// parameters wire field.
    pub parameters: serde_json::Value,
}
