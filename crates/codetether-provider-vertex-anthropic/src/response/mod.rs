//! Buffered response parsing and interpretation.
use crate::*;
mod content;
mod finish;
mod usage;
pub(crate) mod wire;
pub(crate) fn parse(text: &str) -> Result<CompletionResponse> {
    let response: AnthropicResponse = serde_json::from_str(text).context(format!(
        "Failed to parse Vertex AI Anthropic response: {}",
        codetether_util::truncate_bytes_safe(text, 200)
    ))?;

    let (content, has_tool_calls) = content::content(&response);
    let finish_reason = finish::finish(&response, has_tool_calls);
    let usage = response.usage.as_ref();
    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: usage::usage(usage),
        finish_reason,
    })
}
