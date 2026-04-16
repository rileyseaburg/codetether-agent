use crate::provider::{ContentPart, Message, ToolDefinition};
use crate::rlm::RlmChunker;

/// Return the context window size (in tokens) for known models.
pub fn context_window_for_model(model: &str) -> usize {
    let m = model.to_ascii_lowercase();
    if m.contains("kimi-k2") {
        256_000
    } else if m.contains("glm-5") || m.contains("glm5") {
        200_000
    } else if m.contains("gpt-4o") {
        128_000
    } else if m.contains("gpt-5") {
        256_000
    } else if m.contains("claude-opus-4-7") || m.contains("claude-opus-4.7") || m.contains("4.7-opus") {
        1_000_000
    } else if m.contains("claude") {
        200_000
    } else if m.contains("gemini") {
        1_000_000
    } else if m.contains("minimax") || m.contains("m2.5") {
        256_000
    } else if m.contains("qwen") {
        131_072
    } else {
        128_000 // conservative default
    }
}

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
