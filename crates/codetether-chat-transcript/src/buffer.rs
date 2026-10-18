//! Accumulate adjacent text and flush it into a role-appropriate chat row.
use codetether_chat_model::{ChatMessage, MessageType};
use codetether_provider_core::Role;

pub(crate) fn flush_text_buffer(
    role: Role,
    text_buffer: &mut String,
    chat_messages: &mut Vec<ChatMessage>,
) {
    if text_buffer.trim().is_empty() {
        text_buffer.clear();
        return;
    }

    chat_messages.push(ChatMessage::new(
        message_type(role),
        std::mem::take(text_buffer),
    ));
}

pub(crate) fn push_line(out: &mut String, text: &str) {
    if !out.is_empty() {
        out.push('\n');
    }
    out.push_str(text);
}

fn message_type(role: Role) -> MessageType {
    match role {
        Role::User => MessageType::User,
        Role::Assistant => MessageType::Assistant,
        Role::System | Role::Developer | Role::Tool => MessageType::System,
    }
}
