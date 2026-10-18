//! Group adjacent tool-activity messages into [`super::RenderEntry`] batches.

use codetether_chat_model::{ChatMessage, MessageType};

use super::{RenderEntry, is_tool_activity};

///
/// # Arguments
/// * `messages` - Chronological chat messages.
///
/// # Returns
/// Entries grouping contiguous tool activity with the following chat message.
///
/// # Examples
/// ```
/// assert!(codetether_tool_panel::build_render_entries(&[]).is_empty());
/// ```
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

///
/// # Arguments
/// * `entry` - Entry whose message classification determines the separator.
///
/// # Returns
/// The system/tool dot pattern or ordinary line pattern.
///
/// # Examples
/// ```
/// assert_eq!(codetether_tool_panel::separator_pattern(&Default::default()), "·");
/// ```
pub fn separator_pattern(entry: &RenderEntry<'_>) -> &'static str {
    match entry.message.map(|message| &message.message_type) {
        Some(MessageType::System | MessageType::Error) | None => "·",
        _ => "─",
    }
}
