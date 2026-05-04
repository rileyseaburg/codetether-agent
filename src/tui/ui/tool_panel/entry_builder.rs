//! Group adjacent tool-activity messages into [`super::RenderEntry`] batches.

use crate::tui::chat::message::{ChatMessage, MessageType};

use super::{RenderEntry, is_tool_activity};

pub fn build_render_entries(messages: &[ChatMessage]) -> Vec<RenderEntry<'_>> {
    let mut entries = Vec::new();
    let mut pending_tool_activity = Vec::new();

    for message in messages {
        if is_tool_activity(&message.message_type) {
            pending_tool_activity.push(message);
            continue;
        }
        entries.push(RenderEntry {
            tool_activity: std::mem::take(&mut pending_tool_activity),
            message: Some(message),
        });
    }

    if !pending_tool_activity.is_empty() {
        entries.push(RenderEntry {
            tool_activity: pending_tool_activity,
            message: None,
        });
    }

    entries
}

pub fn separator_pattern(entry: &RenderEntry<'_>) -> &'static str {
    match entry.message.map(|message| &message.message_type) {
        Some(MessageType::System | MessageType::Error) | None => "·",
        _ => "─",
    }
}
