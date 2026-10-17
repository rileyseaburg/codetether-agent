//! Completion-request construction from a prepared message projection.
use super::*;
/// Assemble a request using the existing model, tool and sampling options.
///
/// # Arguments
/// * `system_prompt` - Stable system prefix.
/// * `derived` - Selected conversation messages.
/// * `model` - Provider model identifier.
/// * `tools` - Available tool definitions.
/// * `opts` - Request sampling and output options.
///
/// # Returns
/// A request ready for final budget guarding.
///
/// # Examples
/// ```
/// use codetether_context_policy::{request::from_derived, derivation::options::RequestOptions};
/// let request = from_derived("instructions", Vec::new(), "mistral", &[], RequestOptions::default());
/// assert_eq!(request.messages.len(), 1);
/// ```
pub fn from_derived(
    system_prompt: &str,
    derived: Vec<Message>,
    model: &str,
    tools: &[ToolDefinition],
    opts: RequestOptions,
) -> CompletionRequest {
    CompletionRequest {
        messages: messages::build(system_prompt, derived),
        tools: tools.to_vec(),
        model: model.to_string(),
        temperature: opts.temperature,
        top_p: opts.top_p,
        max_tokens: opts.max_tokens,
        stop: Vec::new(),
    }
}
