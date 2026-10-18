//! Join text and thinking parts exactly as returned by the planner.
use codetether_provider_core::ContentPart;
pub(super) fn extract(parts: &[ContentPart]) -> String {
    parts
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } | ContentPart::Thinking { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
