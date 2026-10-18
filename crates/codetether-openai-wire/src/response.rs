//! OpenAI-compatible response wire contracts.
use serde::Serialize;
use crate::{OpenAiChatCompletionMessage, OpenAiUsage};

#[derive(Debug, Serialize)]
/// OpenAI-compatible OpenAiChatCompletionResponse wire representation.
/// # Examples
/// ```
/// let _type_name = std::any::type_name::<codetether_openai_wire::OpenAiChatCompletionResponse>();
/// ```
pub struct OpenAiChatCompletionResponse {
    /// id wire field.
    pub id: String,
    /// object wire field.
    pub object: String,
    /// created wire field.
    pub created: i64,
    /// model wire field.
    pub model: String,
    /// choices wire field.
    pub choices: Vec<OpenAiChatCompletionChoice>,
    /// usage wire field.
    pub usage: OpenAiUsage,
}

#[derive(Debug, Serialize)]
/// OpenAI-compatible OpenAiChatCompletionChoice wire representation.
/// # Examples
/// ```
/// let _type_name = std::any::type_name::<codetether_openai_wire::OpenAiChatCompletionChoice>();
/// ```
pub struct OpenAiChatCompletionChoice {
    /// index wire field.
    pub index: usize,
    /// message wire field.
    pub message: OpenAiChatCompletionMessage,
    /// finish_reason wire field.
    pub finish_reason: String,
}
