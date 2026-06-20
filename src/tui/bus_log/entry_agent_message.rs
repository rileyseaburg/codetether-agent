//! Entry builder for free-form agent messages.

use ratatui::style::Color;

use crate::a2a::types::Part;

use super::{entry_parts::EntryParts, truncate::truncate};

pub(super) fn message(from: &str, to: &str, parts: &[Part]) -> EntryParts {
    let text_preview: String = parts
        .iter()
        .filter_map(|part| match part {
            Part::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ");
    let preview = truncate(&text_preview, 80);
    let a2a = from == "remote-a2a" || to == "remote-a2a";
    let kind = if a2a { "A2A•MSG" } else { "MSG" };
    let detail =
        crate::tui::bus_log_entry_payload::message(from, to, a2a, parts.len(), &text_preview);
    EntryParts::new(
        kind,
        format!("{from} → {to}: {preview}"),
        detail,
        Color::Cyan,
    )
}
