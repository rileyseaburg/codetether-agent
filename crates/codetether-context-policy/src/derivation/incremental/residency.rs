//! Classify rendered summaries for derived-message residency.
use super::*;

pub(super) fn residency_for_message(msg: &Message) -> ResidencyLevel {
    let text = msg
        .content
        .iter()
        .filter_map(|part| match part {
            codetether_provider_core::ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .next()
        .unwrap_or_default();
    if text.starts_with("[SUMMARY of turns ") {
        ResidencyLevel::Compressed
    } else {
        ResidencyLevel::Full
    }
}
