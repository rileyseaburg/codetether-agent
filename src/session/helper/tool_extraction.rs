//! Salvage a tool call from assistant prose when a flaky provider narrates an
//! action instead of emitting a native tool call.
//!
//! This is a best-effort recovery for models (notably MiniMax) that reliably
//! *describe* a tool step ("I'll use bash to run `ls -la`") but intermittently
//! fail to emit the structured `tool_use` block. Extracting the intent keeps
//! the agent loop moving instead of stalling on a corrective retry.

use crate::{
    provider::{CompletionResponse, ContentPart, ToolDefinition},
    session::helper::provider::provider_has_flaky_native_tool_calling,
};

/// Inject a synthetic tool call into `response`/`tool_calls` when a flaky
/// provider narrated an action without emitting a native call.
///
/// No-op unless `tool_calls` and `truncated` are both empty and the provider is
/// known-flaky. Healthy providers and well-formed responses are untouched.
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
    tracing::warn!(provider, tool = %name, "Salvaged synthetic tool call from prose");
    response.message.content.push(ContentPart::ToolCall {
        id: id.clone(),
        name: name.clone(),
        arguments: serde_json::to_string(&args).unwrap_or_else(|_| "{}".to_string()),
        thought_signature: None,
    });
    tool_calls.push((id, name, args));
}

/// Attempt to build a synthetic `(id, name, arguments)` tool call from prose.
///
/// Returns `None` when no high-confidence extraction is possible. Specializes
/// in `bash` (the most common narrated action); other tools are deferred to the
/// retry path to avoid fabricating arguments for schemas we cannot infer.
pub fn extract_tool_call_from_prose(
    text: &str,
    tool_definitions: &[ToolDefinition],
) -> Option<(String, String, serde_json::Value)> {
    let lower = text.to_ascii_lowercase();
    let has_bash = tool_definitions.iter().any(|t| t.name == "bash");
    if !has_bash || !lower.contains("bash") {
        return None;
    }
    let cmd = super::tool_extraction_bash::extract_bash_command(text)?;
    let id = format!("synthetic_{}", &uuid::Uuid::new_v4().to_string()[..8]);
    Some((
        id,
        "bash".to_string(),
        serde_json::json!({ "command": cmd }),
    ))
}
