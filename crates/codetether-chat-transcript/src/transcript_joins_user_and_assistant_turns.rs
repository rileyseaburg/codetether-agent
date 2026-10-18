//! Existing export regression case.
use crate::build_transcript;
use codetether_chat_model::{ChatMessage, MessageType};

fn msg(t: MessageType, c: &str) -> ChatMessage {
    ChatMessage::new(t, c)
}

#[test]
fn transcript_joins_user_and_assistant_turns() {
    let messages = vec![
        msg(MessageType::User, "hello"),
        msg(MessageType::Assistant, "hi there"),
        msg(MessageType::User, "follow up"),
        msg(MessageType::Assistant, "answer"),
    ];
    let out = build_transcript(&messages);
    assert!(out.contains("=== You ===\nhello"));
    assert!(out.contains("=== Assistant ===\nhi there"));
    assert!(out.contains("=== You ===\nfollow up"));
    assert!(out.contains("=== Assistant ===\nanswer"));
    // Two blank lines between turns.
    assert!(out.contains("hi there\n\n=== You ==="));
}
