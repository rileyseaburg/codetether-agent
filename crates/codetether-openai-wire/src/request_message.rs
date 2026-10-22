//! OpenAI-compatible request message wire contracts.
use crate::OpenAiRequestToolCall;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// OpenAI-compatible OpenAiRequestMessage wire representation.
/// # Examples
/// ```
/// let value = serde_json::from_str::<codetether_openai_wire::OpenAiRequestMessage>(r#"{"role":"user","content":"hello"}"#).unwrap();
/// let _ = value;
/// ```
pub struct OpenAiRequestMessage {
    /// role wire field.
    pub role: String,
    #[serde(default)]
    /// content wire field.
    pub content: Option<serde_json::Value>,
    #[serde(default)]
    /// tool_call_id wire field.
    pub tool_call_id: Option<String>,
    #[serde(default)]
    /// tool_calls wire field.
    pub tool_calls: Vec<OpenAiRequestToolCall>,
}
