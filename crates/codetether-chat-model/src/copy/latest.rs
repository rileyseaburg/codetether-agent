//! Select the newest nonempty raw payload for a copy target.
use super::Target;
use crate::{ChatMessage, MessageType};

/// Select the latest nonblank raw payload matching a copy target.
/// # Arguments
/// * `messages` - Messages in chronological order.
/// * `target` - Requested payload kind.
/// # Returns
/// A borrowed raw payload, or None when no message qualifies.
/// # Examples
/// ```
/// use codetether_chat_model::copy::{latest, Target};
/// assert_eq!(latest(&[], Target::Reply), None);
/// ```
pub fn latest(messages: &[ChatMessage], target: Target) -> Option<&str> {
    messages.iter().rev().find_map(|message| {
        let text = match (&message.message_type, target) {
            (MessageType::Assistant, Target::Reply) => &message.content,
            (MessageType::ToolResult { output, .. }, Target::Tool)
            | (
                MessageType::ToolResult {
                    output,
                    success: false,
                    ..
                },
                Target::Error,
            ) => output,
            (MessageType::Error, Target::Error) => &message.content,
            _ => return None,
        };
        (!text.trim().is_empty()).then_some(text.as_str())
    })
}
