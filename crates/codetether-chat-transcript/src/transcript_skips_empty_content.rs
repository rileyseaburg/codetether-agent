//! Existing export regression case.
use crate::build_transcript;
use codetether_chat_model::{ChatMessage, MessageType};

fn msg(t: MessageType, c: &str) -> ChatMessage {
    ChatMessage::new(t, c)
}

#[test]
fn transcript_skips_empty_content() {
    let messages = vec![
        msg(MessageType::User, "   \n  "),
        msg(MessageType::Assistant, "real reply"),
    ];
    let out = build_transcript(&messages);
    assert_eq!(out, "=== Assistant ===\nreal reply");
}
