//! decision for Z.AI thinking requests.
use super::instruction::contains_explicit_short_answer_instruction;
use crate::{CompletionRequest, ContentPart};
/// Decide whether a request uses the existing short-answer policy.
///
/// # Arguments
/// * request - Conversation messages and output-token limit.
///
/// # Returns
/// True for explicit short-answer instructions, or short text with at most 256
/// output tokens; false otherwise.
///
/// # Examples
///
///     use codetether_provider_zai::{CompletionRequest, thinking};
///     fn decision(request: &CompletionRequest) -> bool {
///         thinking::should_disable_thinking_for_request(request)
///     }
pub fn should_disable_thinking_for_request(request: &CompletionRequest) -> bool {
    if request
        .messages
        .iter()
        .any(contains_explicit_short_answer_instruction)
    {
        return true;
    }

    request.max_tokens.is_some_and(|max| max <= 256)
        && request.messages.iter().any(|message| {
            message.content.iter().any(|part| match part {
                ContentPart::Text { text } => text.chars().count() <= 512,
                _ => false,
            })
        })
}
