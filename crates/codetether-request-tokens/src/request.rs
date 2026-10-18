//! Combine prompt, message and tool estimates for a request.
use crate::{estimate_tokens_for_messages, estimate_tokens_for_tools};
use codetether_provider_core::{Message, ToolDefinition};
use codetether_rlm::RlmChunker;

/// Combine prompt, transcript and tool-definition token costs.
/// # Arguments
/// * `system_prompt` — System instructions for the request.
/// * `messages` — Conversation transcript.
/// * `tools` — Tool definitions available to the model.
/// # Returns
/// The sum of prompt, message and tool estimates.
/// # Examples
/// ```
/// let estimate = codetether_request_tokens::estimate_request_tokens("A sufficiently long system prompt", &[], &[]);
/// assert!(estimate > 0);
/// ```
pub fn estimate_request_tokens(
    system_prompt: &str,
    messages: &[Message],
    tools: &[ToolDefinition],
) -> usize {
    RlmChunker::estimate_tokens(system_prompt)
        + estimate_tokens_for_messages(messages)
        + estimate_tokens_for_tools(tools)
}
