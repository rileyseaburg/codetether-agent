//! Construct the provider request for the next sub-agent turn.
use codetether_provider_core::{CompletionRequest, Message, ToolDefinition};

/// Build a completion request from current sub-agent state.
/// # Arguments
/// Supply current messages, advertised tools, model, temperature, and token limit.
/// # Returns
/// An owned request with no top-p override or stop sequences.
/// # Examples
/// ```
/// let request = codetether_swarm_core::agent_loop::request::build(&[], &[], "model", None, 1024);
/// assert_eq!(request.max_tokens, Some(1024));
/// assert!(request.stop.is_empty());
/// ```
pub fn build(
    messages: &[Message],
    tools: &[ToolDefinition],
    model: &str,
    temperature: Option<f32>,
    max_tokens: usize,
) -> CompletionRequest {
    CompletionRequest {
        messages: messages.to_vec(),
        tools: tools.to_vec(),
        model: model.to_string(),
        temperature,
        top_p: None,
        max_tokens: Some(max_tokens),
        stop: Vec::new(),
    }
}
