//! Message tags and text formatting for training records.
use super::*;
#[cfg(test)]
#[path = "format_tests.rs"]
mod tests;

/// Extract the serde tag name from a `BusMessage` variant.
pub(super) fn bus_message_kind(msg: &BusMessage) -> String {
    serde_json::to_value(msg)
        .ok()
        .and_then(|v| v.get("kind").and_then(|k| k.as_str()).map(String::from))
        .unwrap_or_else(|| "unknown".into())
}

/// Concatenate `Part` items into a single text string.
pub(super) fn parts_to_text(parts: &[Part]) -> String {
    parts
        .iter()
        .map(|p| match p {
            Part::Text { text } => text.as_str(),
            Part::Data { .. } => "<<data>>",
            Part::File { .. } => "<<file>>",
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Format an agent message without polluting user-facing assistant text.
pub(super) fn agent_message_text(from: &str, to: &str, text: &str) -> String {
    if to == "user" {
        text.to_string()
    } else {
        format!("[{from} → {to}] {text}")
    }
}
