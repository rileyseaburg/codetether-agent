//! Existing export regression case.
use crate::build_transcript;
use codetether_chat_model::{ChatMessage, MessageType};

fn msg(t: MessageType, c: &str) -> ChatMessage {
    ChatMessage::new(t, c)
}

#[test]
fn transcript_skips_internal_message_types() {
    let messages = vec![
        msg(MessageType::User, "hello"),
        msg(MessageType::Thinking("inner monologue".into()), ""),
        msg(
            MessageType::ToolCall {
                name: "shell".into(),
                arguments: "ls".into(),
            },
            "",
        ),
        msg(MessageType::Assistant, "hi"),
    ];
    let out = build_transcript(&messages);
    assert!(!out.contains("inner monologue"));
    assert!(!out.contains("shell"));
    assert!(!out.contains("Tool"));
    assert_eq!(out, "=== You ===\nhello\n\n=== Assistant ===\nhi");
}
