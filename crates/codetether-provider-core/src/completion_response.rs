use crate::{FinishReason, Message, Usage};

/// Response from a completion request.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::{CompletionResponse, FinishReason, Message, Role, Usage};
/// let response = CompletionResponse {
///     message: Message { role: Role::Assistant, content: vec![] },
///     usage: Usage::default(), finish_reason: FinishReason::Stop,
/// };
/// assert_eq!(response.finish_reason, FinishReason::Stop);
/// ```
#[derive(Debug, Clone)]
pub struct CompletionResponse {
    /// The model's reply message.
    pub message: Message,
    /// Token usage for this request.
    pub usage: Usage,
    /// Why the model stopped generating.
    pub finish_reason: FinishReason,
}
