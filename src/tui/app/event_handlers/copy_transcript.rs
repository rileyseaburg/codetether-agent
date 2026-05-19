//! Ctrl+Shift+Y handler: copy the entire conversation transcript.
//!
//! Mouse-selection in the chat panel captures the rendered box
//! borders and trailing whitespace because the terminal does not
//! know about the [`Block`](ratatui::widgets::Block) layout. This
//! handler bypasses that limitation by writing a clean plain-text
//! transcript directly to the system clipboard.

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::clipboard_text::copy_text;
use crate::tui::constants::SCROLL_BOTTOM;

/// Build a plain-text transcript from a slice of chat messages.
///
/// Includes only `User` and `Assistant` turns. Tool calls, tool
/// results, thinking blocks, images, and file attachments are
/// dropped — the transcript is intended for sharing the
/// conversation, not the agent's internal scratch work.
pub(crate) fn build_transcript(messages: &[ChatMessage]) -> String {
    let mut out = String::new();
    for msg in messages {
        let role = match &msg.message_type {
            MessageType::User => "You",
            MessageType::Assistant => "Assistant",
            _ => continue,
        };
        let content = msg.content.trim();
        if content.is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push_str("\n\n");
        }
        out.push_str("=== ");
        out.push_str(role);
        out.push_str(" ===\n");
        out.push_str(content);
    }
    out
}

pub(super) fn handle_copy_transcript(app: &mut App) {
    let transcript = build_transcript(&app.state.messages);
    if transcript.is_empty() {
        app.state.status = "No conversation to copy yet.".into();
        return;
    }
    let turns = transcript.matches("=== ").count();
    match copy_text(&transcript) {
        Ok(method) => {
            app.state.status = format!("Copied transcript: {turns} turns ({method}).");
            app.state.chat_scroll = SCROLL_BOTTOM;
        }
        Err(err) => {
            tracing::warn!(error = %err, "Copy transcript to clipboard failed");
            app.state.status = "Could not copy transcript to clipboard.".into();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn transcript_skips_empty_content() {
        let messages = vec![
            msg(MessageType::User, "   \n  "),
            msg(MessageType::Assistant, "real reply"),
        ];
        let out = build_transcript(&messages);
        assert_eq!(out, "=== Assistant ===\nreal reply");
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

    #[test]
    fn transcript_empty_for_empty_history() {
        assert_eq!(build_transcript(&[]), "");
    }
}
