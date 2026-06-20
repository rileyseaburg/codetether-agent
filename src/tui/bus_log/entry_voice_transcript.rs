//! Entry builder for voice transcript messages.

use ratatui::style::Color;

use super::{entry_parts::EntryParts, truncate::truncate};

pub(super) fn transcript(room_name: &str, text: &str, role: &str, is_final: bool) -> EntryParts {
    let fin = if is_final { " [final]" } else { "" };
    let detail = crate::tui::bus_log_entry_payload::transcript(room_name, role, is_final, text);
    EntryParts::new(
        "VOICE•T",
        format!("{room_name} [{role}]{fin}: {}", truncate(text, 100)),
        detail,
        Color::LightCyan,
    )
}
