//! OpenAI-compatible request call wire contracts.
use crate::defaults::{default_function_type, default_json_object_string};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// OpenAI-compatible OpenAiRequestToolCall wire representation.
/// # Examples
/// ```
/// let value = serde_json::from_str::<codetether_openai_wire::OpenAiRequestToolCall>(r#"{"id":"call","function":{"name":"read"}}"#).unwrap();
/// let _ = value;
/// ```
pub struct OpenAiRequestToolCall {
    /// id wire field.
    pub id: String,
    #[serde(rename = "type", default = "default_function_type")]
    /// kind wire field.
    pub kind: String,
    /// function wire field.
    pub function: OpenAiRequestToolCallFunction,
}

#[derive(Debug, Deserialize)]
/// OpenAI-compatible OpenAiRequestToolCallFunction wire representation.
/// # Examples
/// ```
/// let value = serde_json::from_str::<codetether_openai_wire::OpenAiRequestToolCallFunction>(r#"{"name":"read"}"#).unwrap();
/// let _ = value;
/// ```
pub struct OpenAiRequestToolCallFunction {
    /// name wire field.
    pub name: String,
    #[serde(default = "default_json_object_string")]
    /// arguments wire field.
    pub arguments: String,
}
