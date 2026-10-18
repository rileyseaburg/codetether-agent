//! Append a recovered call only when native call extraction produced none.
use crate::extract_tool_call_from_prose;
use codetether_provider_core::{CompletionResponse, ContentPart, ToolDefinition};
use codetether_provider_policy::provider_has_flaky_native_tool_calling;

/// Inject a synthetic tool call into `response`/`tool_calls` when a flaky
/// provider narrated an action without emitting a native call.
///
/// No-op unless `tool_calls` and `truncated` are both empty and the provider is
/// known-flaky. Healthy providers and well-formed responses are untouched.
/// # Arguments
/// Receives provider/model identity, text, registered tools, truncated IDs and mutable response/calls.
/// # Returns
/// Mutates response and call list only when recovery succeeds.
/// # Examples
/// ```
/// # fn example(response: &mut codetether_provider_core::CompletionResponse) {
/// let mut calls = vec![];
/// codetether_tool_recovery::salvage_prose_tool_call("openai", "model", "", &[], &[], response, &mut calls);
/// assert!(calls.is_empty());
/// # }
/// ```
pub fn salvage_prose_tool_call(
    provider: &str,
    model: &str,
    assistant_text: &str,
    tools: &[ToolDefinition],
    truncated: &[(String, String)],
    response: &mut CompletionResponse,
    tool_calls: &mut Vec<(String, String, serde_json::Value)>,
) {
    if !tool_calls.is_empty()
        || !truncated.is_empty()
        || !provider_has_flaky_native_tool_calling(provider, model)
    {
        return;
    }
    let Some((id, name, args)) = extract_tool_call_from_prose(assistant_text, tools) else {
        return;
    };
    tracing::warn!(target: "codetether_agent::session::helper::tool_modules::tool_extraction",provider, tool = %name, "Salvaged synthetic tool call from prose");
    response.message.content.push(ContentPart::ToolCall {
        id: id.clone(),
        name: name.clone(),
        arguments: serde_json::to_string(&args).unwrap_or_else(|_| "{}".to_string()),
        thought_signature: None,
    });
    tool_calls.push((id, name, args));
}
