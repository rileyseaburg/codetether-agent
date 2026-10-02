//! Clipboard payload contracts; no OS clipboard is needed.

use super::{Target, latest};
use crate::tui::chat::message::{ChatMessage, MessageType};

#[path = "copy_target_tests.rs"]
mod target_tests;

fn tool(output: &str, success: bool) -> ChatMessage {
    ChatMessage::new(
        MessageType::ToolResult {
            name: "exec_command".into(),
            output: output.into(),
            success,
            duration_ms: Some(13732),
        },
        "preview must not be copied",
    )
}

#[test]
fn reply_copy_preserves_exact_raw_content_and_skips_system() {
    let messages = [
        ChatMessage::new(MessageType::Assistant, "  code\n\n"),
        ChatMessage::new(MessageType::System, "Approved once"),
    ];
    assert_eq!(latest(&messages, Target::Reply), Some("  code\n\n"));
}

#[test]
fn tool_copy_omits_timestamp_name_duration_and_preview() {
    let raw = "Permission denied (os error 13)\nCaused by: cache is read-only";
    assert_eq!(latest(&[tool(raw, false)], Target::Tool), Some(raw));
}

#[test]
fn error_copy_skips_later_success_status_and_empty_errors() {
    let messages = [
        tool("sandbox failure\nunderlying cause", false),
        tool("ok", true),
        ChatMessage::new(MessageType::System, "ready"),
        tool(" ", false),
    ];
    assert_eq!(
        latest(&messages, Target::Error),
        Some("sandbox failure\nunderlying cause")
    );
    assert_eq!(latest(&[], Target::Tool), None);
}
