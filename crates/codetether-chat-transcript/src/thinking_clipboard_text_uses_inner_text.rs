//! Existing clipboard regression case.
use crate::clipboard_text;
use codetether_chat_model::{ChatMessage, MessageType};

#[test]
fn thinking_clipboard_text_uses_inner_text() {
    let msg = ChatMessage::new(MessageType::Thinking("hidden".into()), "");
    assert_eq!(clipboard_text(&msg), "hidden");
}
