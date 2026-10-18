//! Locate reverse user-turn boundaries.
use crate::turn_undo::is_user_message;
use crate::turn_undo_count::is_ui_user;
use codetether_chat_model::ChatMessage;
use codetether_provider_core::Message;

pub(crate) fn session_cut(messages: &[Message], count: usize) -> Option<usize> {
    messages
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(idx, msg)| is_user_message(msg).then_some(idx))
        .nth(count - 1)
}

pub(crate) fn ui_cut(messages: &[ChatMessage], count: usize) -> Option<usize> {
    messages
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(idx, msg)| is_ui_user(msg).then_some(idx))
        .nth(count - 1)
}
