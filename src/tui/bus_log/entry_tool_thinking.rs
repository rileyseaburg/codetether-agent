//! Entry builder for retained agent thinking messages.

use ratatui::style::Color;

use super::{entry_parts::EntryParts, truncate::truncate};

pub(super) fn thinking(agent_id: &str, thinking: &str, step: usize) -> EntryParts {
    let preview = truncate(thinking, 120);
    let detail = crate::tui::bus_log_entry_payload::thinking(agent_id, step, thinking);
    EntryParts::new(
        "THINK",
        format!("{agent_id} step {step}: {preview}"),
        detail,
        Color::LightMagenta,
    )
}
