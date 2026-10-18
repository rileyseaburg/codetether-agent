//! OpenAI-compatible response call wire contracts.
use serde::Serialize;

#[derive(Debug, Serialize)]
/// OpenAI-compatible OpenAiResponseToolCall wire representation.
/// # Examples
/// ```
/// let _type_name = std::any::type_name::<codetether_openai_wire::OpenAiResponseToolCall>();
/// ```
pub struct OpenAiResponseToolCall {
    /// id wire field.
    pub id: String,
    #[serde(rename = "type")]
    /// kind wire field.
    pub kind: String,
    /// function wire field.
    pub function: OpenAiResponseToolCallFunction,
}

#[derive(Debug, Serialize)]
/// OpenAI-compatible OpenAiResponseToolCallFunction wire representation.
/// # Examples
/// ```
/// let _type_name = std::any::type_name::<codetether_openai_wire::OpenAiResponseToolCallFunction>();
/// ```
pub struct OpenAiResponseToolCallFunction {
    /// name wire field.
    pub name: String,
    /// arguments wire field.
    pub arguments: String,
}
