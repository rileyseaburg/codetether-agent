//! Collect text content from an LLM response in its original order.

use codetether_provider_core::{CompletionResponse, ContentPart};
pub(crate) fn collect(response: &CompletionResponse) -> String {
    response
        .message
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("")
}
