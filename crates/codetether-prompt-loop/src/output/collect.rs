//! Collect nonempty text or reasoning segments in response order.
use codetether_provider_core::ContentPart;

pub(super) fn collect(parts: &[ContentPart], thinking: bool) -> String {
    parts
        .iter()
        .filter_map(|part| match (part, thinking) {
            (ContentPart::Thinking { text, .. }, true) => Some(text.as_str()),
            (ContentPart::Text { text }, false) => Some(text.as_str()),
            _ => None,
        })
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
