//! Registry-provider response conversion for cognition inference.

use codetether_provider_core::{CompletionResponse, ContentPart, FinishReason};

use super::ThinkerOutput;

///
/// # Arguments
/// * `model` - Resolved registry model identifier.
/// * `response` - Completed provider response.
///
/// # Returns
/// The constructed request or normalized thinker output.
///
/// # Examples
/// ```
/// # use codetether_cognition_core::thinker::{ThinkerConfig, ThinkerOutput};
/// # use codetether_cognition_core::thinker::protocol::openai_wire::*;
/// # use codetether_provider_core::CompletionResponse;
/// # fn example(model: &str, response: CompletionResponse) {
/// let _result = codetether_cognition_core::thinker::protocol::provider_output::convert(model, response);
/// # }
/// ```
pub fn convert(model: &str, response: CompletionResponse) -> ThinkerOutput {
    let text = response
        .message
        .content
        .into_iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    ThinkerOutput {
        model: model.into(),
        finish_reason: Some(reason(response.finish_reason).into()),
        text,
        prompt_tokens: count(response.usage.prompt_tokens),
        completion_tokens: count(response.usage.completion_tokens),
        total_tokens: count(response.usage.total_tokens),
        cache_read_tokens: response.usage.cache_read_tokens.and_then(count),
        cache_write_tokens: response.usage.cache_write_tokens.and_then(count),
    }
}

fn reason(reason: FinishReason) -> &'static str {
    match reason {
        FinishReason::Stop => "stop",
        FinishReason::Length => "length",
        FinishReason::ToolCalls => "tool_calls",
        FinishReason::ContentFilter => "content_filter",
        FinishReason::Error => "error",
    }
}

fn count(value: usize) -> Option<u32> {
    Some(u32::try_from(value).unwrap_or(u32::MAX))
}
