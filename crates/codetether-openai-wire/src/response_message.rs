//! OpenAI-compatible response message wire contracts.
use crate::OpenAiResponseToolCall;
use serde::Serialize;

#[derive(Debug, Serialize)]
/// OpenAI-compatible OpenAiChatCompletionMessage wire representation.
/// # Examples
/// ```
/// let _type_name = std::any::type_name::<codetether_openai_wire::OpenAiChatCompletionMessage>();
/// ```
pub struct OpenAiChatCompletionMessage {
    /// role wire field.
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// content wire field.
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// tool_calls wire field.
    pub tool_calls: Option<Vec<OpenAiResponseToolCall>>,
}
