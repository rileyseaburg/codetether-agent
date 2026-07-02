//! Parse native Anthropic Messages responses into [`CompletionResponse`].

use super::types::AnthropicMessagesResponse;
use crate::provider::bedrock::empty_guard;
use crate::provider::bedrock::invoke::response::content::map_content;
use crate::provider::{CompletionResponse, FinishReason, Message, Role, Usage};
use anyhow::{Context, Result};

pub(in crate::provider::bedrock) fn parse_anthropic_messages_response(
    text: &str,
) -> Result<CompletionResponse> {
    let response: AnthropicMessagesResponse = serde_json::from_str(text).context(format!(
        "Failed to parse Bedrock InvokeModel response: {}",
        crate::util::truncate_bytes_safe(text, 300)
    ))?;

    let (content, has_tool_calls) = map_content(response.content);
    let finish_reason = if has_tool_calls {
        FinishReason::ToolCalls
    } else {
        empty_guard::check_visible_output(&content, response.stop_reason.as_deref())?;
        match response.stop_reason.as_deref() {
            Some("end_turn") | Some("stop") | Some("stop_sequence") => FinishReason::Stop,
            Some("max_tokens") => FinishReason::Length,
            Some("tool_use") => FinishReason::ToolCalls,
            Some("content_filtered") => FinishReason::ContentFilter,
            _ => FinishReason::Stop,
        }
    };

    let usage = response.usage.unwrap_or_default();
    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: Usage {
            prompt_tokens: usage.input_tokens,
            completion_tokens: usage.output_tokens,
            total_tokens: usage.input_tokens + usage.output_tokens,
            cache_read_tokens: Some(usage.cache_read_input_tokens),
            cache_write_tokens: Some(usage.cache_creation_input_tokens),
        },
        finish_reason,
    })
}
