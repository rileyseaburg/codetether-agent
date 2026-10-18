//! Independent trailing-turn cut points for persisted and displayed histories.
use codetether_chat_model::{ChatMessage, MessageType};
use codetether_provider_core::{Message, Role};
pub(super) fn session(messages: &[Message], drop_last_n: usize) -> usize {
    let session_user_idxs: Vec<usize> = messages
        .iter()
        .enumerate()
        .filter_map(|(i, m)| (m.role == Role::User).then_some(i))
        .collect();
    if drop_last_n == 0 || drop_last_n > session_user_idxs.len() {
        messages.len()
    } else {
        session_user_idxs[session_user_idxs.len() - drop_last_n]
    }
}
pub(super) fn ui(messages: &[ChatMessage], drop_last_n: usize) -> usize {
    let tui_user_idxs: Vec<usize> = messages
        .iter()
        .enumerate()
        .filter_map(|(i, m)| matches!(m.message_type, MessageType::User).then_some(i))
        .collect();
    if drop_last_n == 0 || drop_last_n > tui_user_idxs.len() {
        messages.len()
    } else {
        tui_user_idxs[tui_user_idxs.len() - drop_last_n]
    }
}
