//! Build provider completion input from a validated wire request.
use crate::{OpenAiApiResult, convert_openai_messages, convert_openai_tools};
use codetether_openai_wire::{OpenAiChatCompletionRequest, OpenAiStop};
/// Convert a wire request into provider input after provider selection.
/// # Arguments
/// * `req` - Incoming wire request.
/// * `model_id` - Selected provider-local model identifier.
/// # Returns
/// Converted messages/tools and original sampling and stop options.
/// # Errors
/// Returns the original message/tool validation error.
/// # Examples
/// ```
/// let req = serde_json::from_value::<codetether_openai_wire::OpenAiChatCompletionRequest>(serde_json::json!({"model":"model","messages":[]})).unwrap();
/// assert_eq!(codetether_openai_adapter::completion_request(req, "model").unwrap().model, "model");
/// ```
pub fn completion_request(
    req: OpenAiChatCompletionRequest,
    model_id: &str,
) -> OpenAiApiResult<codetether_provider_core::CompletionRequest> {
    let messages = convert_openai_messages(&req.messages)?;
    let tools = convert_openai_tools(&req.tools)?;
    let stop = match req.stop {
        Some(OpenAiStop::Single(stop)) => vec![stop],
        Some(OpenAiStop::Multiple(stops)) => stops,
        None => Vec::new(),
    };

    Ok(codetether_provider_core::CompletionRequest {
        messages,
        tools,
        model: model_id.to_string(),
        temperature: req.temperature,
        top_p: req.top_p,
        max_tokens: req.max_completion_tokens.or(req.max_tokens),
        stop,
    })
}
