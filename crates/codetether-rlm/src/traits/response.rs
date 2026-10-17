//! Completion returned by a host model provider.

use super::ToolCall;

/// Completion response from a provider.
///
/// # Examples
/// ```
/// use codetether_rlm::{traits::{LlmMessage,LlmResponse}};
/// let response = LlmResponse { text: "hello".into(), tool_calls: vec![], finish_reason: None, input_tokens: 1, output_tokens: 1 };
/// assert_eq!(LlmMessage::assistant_from(&response).text, "hello");
/// ```
#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub text: String,
    pub tool_calls: Vec<ToolCall>,
    pub finish_reason: Option<String>,
    pub input_tokens: usize,
    pub output_tokens: usize,
}
