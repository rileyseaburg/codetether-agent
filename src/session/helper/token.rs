use crate::provider::{ContentPart, Message, ToolDefinition};
use crate::rlm::RlmChunker;

/// Delegate to the canonical implementation in [`crate::provider::limits`].
///
/// Kept here as a re-export so existing call sites don't need to update
/// their import paths. New code may import
/// [`crate::provider::limits::context_window_for_model`] directly.
pub use crate::provider::limits::context_window_for_model;

pub fn session_completion_max_tokens() -> usize {
    std::env::var("CODETETHER_SESSION_MAX_TOKENS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(8192)
}

pub fn estimate_tokens_for_part(part: &ContentPart) -> usize {
    match part {
        ContentPart::Text { text } => RlmChunker::estimate_tokens(text),
        ContentPart::ToolResult { content, .. } => RlmChunker::estimate_tokens(content),
        ContentPart::ToolCall {
            id,
            name,
            arguments,
            thought_signature,
            ..
        } => {
            let mut s = String::new();
            s.push_str(id);
            s.push(' ');
            s.push_str(name);
            s.push(' ');
            s.push_str(arguments);
            if let Some(sig) = thought_signature {
                s.push(' ');
                s.push_str(sig);
            }
            RlmChunker::estimate_tokens(&s)
        }
        ContentPart::Thinking { text } => RlmChunker::estimate_tokens(text),
        ContentPart::Image { .. } => 2000,
        ContentPart::File { path, mime_type } => {
            let mut s = String::new();
            s.push_str(path);
            if let Some(mt) = mime_type {
                s.push(' ');
                s.push_str(mt);
            }
            RlmChunker::estimate_tokens(&s)
        }
    }
}

pub fn estimate_tokens_for_messages(messages: &[Message]) -> usize {
    messages
        .iter()
        .map(|m| {
            m.content
                .iter()
                .map(estimate_tokens_for_part)
                .sum::<usize>()
        })
        .sum()
}

pub fn estimate_tokens_for_tools(tools: &[ToolDefinition]) -> usize {
    serde_json::to_string(tools)
        .ok()
        .map(|s| RlmChunker::estimate_tokens(&s))
        .unwrap_or(0)
}

pub fn estimate_request_tokens(
    system_prompt: &str,
    messages: &[Message],
    tools: &[ToolDefinition],
) -> usize {
    RlmChunker::estimate_tokens(system_prompt)
        + estimate_tokens_for_messages(messages)
        + estimate_tokens_for_tools(tools)
}
