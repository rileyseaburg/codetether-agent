use crate::provider::Message;
use crate::tui::app::turn_undo_registration::turn_undo_mods::turn_undo::is_user_message;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(crate) fn count_session_users(messages: &[Message]) -> usize {
    messages.iter().filter(|m| is_user_message(m)).count()
}

pub(crate) fn count_ui_users(messages: &[ChatMessage]) -> usize {
    messages.iter().filter(|m| is_ui_user(m)).count()
}

pub(crate) fn is_ui_user(message: &ChatMessage) -> bool {
    matches!(message.message_type, MessageType::User)
}
