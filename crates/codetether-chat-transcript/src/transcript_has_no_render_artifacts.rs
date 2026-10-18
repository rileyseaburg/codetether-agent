//! Existing export regression case.
use crate::build_transcript;
use codetether_chat_model::{ChatMessage, MessageType};

fn msg(t: MessageType, c: &str) -> ChatMessage {
    ChatMessage::new(t, c)
}

#[test]
fn transcript_has_no_render_artifacts() {
    let raw = "- Local Qwen TTS service is running:\n  - http://127.0.0.1:8015\n- Done.";
    let messages = vec![msg(MessageType::Assistant, raw)];
    let out = build_transcript(&messages);
    for forbidden in ["│", "┌", "└", "─", "┐", "┘"] {
        assert!(
            !out.contains(forbidden),
            "transcript leaked render border {forbidden:?}"
        );
    }
    assert!(out.contains(raw));
}
