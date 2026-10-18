//! Existing clipboard regression case.
use crate::clipboard_text;
use codetether_chat_model::{ChatMessage, MessageType};

/// `clipboard_text` for an assistant message must return the raw
/// `content` exactly — no box-drawing characters, no trailing
/// whitespace from the rendered chat panel. Borders are added at
/// render time only, so they must never appear in the clipboard
/// payload regardless of how the message is later wrapped on screen.
#[test]
fn assistant_clipboard_text_has_no_render_artifacts() {
    let raw = "- Local Qwen TTS service is running:\n  - http://127.0.0.1:8015\n- Done.";
    let msg = ChatMessage::new(MessageType::Assistant, raw);
    let copied = clipboard_text(&msg);
    assert_eq!(copied, raw);
    for forbidden in ["│", "┌", "└", "─", "┐", "┘"] {
        assert!(
            !copied.contains(forbidden),
            "clipboard text leaked render border {forbidden:?}: {copied:?}"
        );
    }
}
