//! Count session and displayed user turns.
use crate::turn_undo::is_user_message;
use codetether_chat_model::{ChatMessage, MessageType};
use codetether_provider_core::Message;

pub(crate) fn count_session_users(messages: &[Message]) -> usize {
    messages.iter().filter(|m| is_user_message(m)).count()
}

pub(crate) fn count_ui_users(messages: &[ChatMessage]) -> usize {
    messages.iter().filter(|m| is_ui_user(m)).count()
}

pub(crate) fn is_ui_user(message: &ChatMessage) -> bool {
    matches!(message.message_type, MessageType::User)
}
