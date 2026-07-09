//! Entry builder for [`BusMessage::UserPrompt`] bus messages.
//!
//! [`BusMessage::UserPrompt`]: crate::bus::BusMessage::UserPrompt

use ratatui::style::Color;

use super::{entry_parts::EntryParts, truncate::truncate};

/// Build the bus-log entry for a user prompt published to the bus.
pub(super) fn user_prompt(
    agent_id: &str,
    text: &str,
    workspace: &str,
    session_id: &str,
) -> EntryParts {
    let preview = truncate(text, 80);
    let detail = format!(
        "agent: {agent_id}\nsession: {session_id}\nworkspace: {workspace}\n\n{text}"
    );
    EntryParts::new(
        "USER",
        format!("{agent_id}: {preview}"),
        detail,
        Color::Green,
    )
}
