//! Convert shared provider completions into RLM responses.

use crate::traits::{LlmResponse, ToolCall};
use codetether_provider_core::{CompletionResponse, ContentPart};

/// Convert text, tool calls, and finish reason into the RLM response contract.
///
/// # Arguments
/// * `resp` - Completed provider response.
/// # Returns
/// Collected text and tool calls, preserving the adapter's zero token counters.
/// # Examples
/// ```no_run
/// # fn example(response: codetether_provider_core::CompletionResponse) {
/// let converted = codetether_rlm::provider_adapter::to_llm_resp(response);
/// assert_eq!(converted.input_tokens, 0);
/// # }
/// ```
pub fn to_llm_resp(resp: CompletionResponse) -> LlmResponse {
    LlmResponse {
        text: text(&resp),
        tool_calls: tool_calls(&resp),
        finish_reason: Some(format!("{:?}", resp.finish_reason)),
        input_tokens: 0,
        output_tokens: 0,
    }
}

fn text(resp: &CompletionResponse) -> String {
    resp.message
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect()
}

fn tool_calls(resp: &CompletionResponse) -> Vec<ToolCall> {
    resp.message
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => Some(ToolCall {
                id: id.clone(),
                name: name.clone(),
                arguments: serde_json::from_str(arguments).unwrap_or_default(),
            }),
            _ => None,
        })
        .collect()
}
