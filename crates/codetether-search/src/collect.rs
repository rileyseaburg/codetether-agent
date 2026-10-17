//! Collect text and thinking output in provider response order.

pub(crate) fn collect_text(response: &codetether_provider_core::CompletionResponse) -> String {
    response
        .message
        .content
        .iter()
        .filter_map(|part| match part {
            codetether_provider_core::ContentPart::Text { text }
            | codetether_provider_core::ContentPart::Thinking { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
