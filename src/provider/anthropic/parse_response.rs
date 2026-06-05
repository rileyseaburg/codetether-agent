//! Conversion from Anthropic response payloads into provider responses.
//!
//! This module adapts parsed Anthropic Messages API response bodies into the
//! provider-neutral [`CompletionResponse`] type used by the rest of the
//! application. It collects assistant content blocks, maps usage accounting,
//! and normalizes provider stop reasons into internal finish reasons.

use crate::provider::{CompletionResponse, FinishReason, Message, Role};

use super::response::AnthropicResponse;

/// Convert a parsed Anthropic response into the generic provider response.
///
/// The response content blocks are converted with
/// [`super::parse_content::content_part`]. Any tool-use content marks the
/// resulting completion as requiring tool calls, even when the provider stop
/// reason is missing or ambiguous. Token usage is copied through
/// [`super::parse_usage::usage`].
///
/// # Parameters
///
/// * `response` - Parsed Anthropic-compatible response body to normalize.
///
/// # Returns
///
/// A [`CompletionResponse`] containing an assistant [`Message`], optional usage
/// accounting, and the normalized [`FinishReason`].
///
/// # Side Effects
///
/// This function performs no I/O. It consumes `response` and clones individual
/// content fields as needed during conversion.
pub(crate) fn parse(response: AnthropicResponse) -> CompletionResponse {
    let mut content = Vec::new();
    let mut has_tool_calls = false;
    for part in &response.content {
        if let Some(converted) = super::parse_content::content_part(part, &mut has_tool_calls) {
            content.push(converted);
        }
    }
    CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: super::parse_usage::usage(response.usage.as_ref()),
        finish_reason: finish_reason(response.stop_reason.as_deref(), has_tool_calls),
    }
}

/// Normalize an Anthropic stop reason into the internal finish reason.
///
/// Tool-call detection from parsed content takes precedence over the textual
/// stop reason because some providers return tool-use blocks without reliably
/// setting `stop_reason` to `tool_use`. Unknown or missing stop reasons default
/// to [`FinishReason::Stop`] to preserve the provider's historical fallback
/// behavior.
///
/// # Parameters
///
/// * `stop_reason` - Optional provider stop reason from the response body.
/// * `has_tool_calls` - Whether converted response content contained tool calls.
///
/// # Returns
///
/// The internal [`FinishReason`] corresponding to the provider stop condition.
fn finish_reason(stop_reason: Option<&str>, has_tool_calls: bool) -> FinishReason {
    if has_tool_calls {
        return FinishReason::ToolCalls;
    }
    match stop_reason {
        Some("end_turn") | Some("stop") => FinishReason::Stop,
        Some("max_tokens") => FinishReason::Length,
        Some("tool_use") => FinishReason::ToolCalls,
        Some("content_filter") => FinishReason::ContentFilter,
        _ => FinishReason::Stop,
    }
}
