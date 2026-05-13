//! Usage badge attachment helper.

use crate::tui::chat::message::{ChatMessage, MessageType, MessageUsage};

/// Attach usage to the newest assistant or tool-call message.
pub fn attach_usage_to_last_completion_message(messages: &mut [ChatMessage], usage: MessageUsage) {
    for msg in messages.iter_mut().rev() {
        if msg.usage.is_some() {
            continue;
        }
        match &msg.message_type {
            MessageType::Assistant | MessageType::ToolCall { .. } => {
                msg.usage = Some(usage);
                return;
            }
            MessageType::User => return,
            _ => {}
        }
    }
}
