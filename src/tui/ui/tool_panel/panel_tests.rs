use super::{TOOL_PANEL_VISIBLE_LINES, build_tool_activity_panel, is_tool_activity};
use crate::tui::chat::message::{ChatMessage, MessageType};

#[test]
fn tool_panel_keeps_stable_height_for_short_activity() {
    let message = ChatMessage::new(
        MessageType::ToolCall {
            name: "bash".to_string(),
            arguments: "{}".to_string(),
        },
        "",
    );
    let panel = build_tool_activity_panel(&[&message], 0, 80, None);
    assert_eq!(panel.lines.len(), TOOL_PANEL_VISIBLE_LINES + 2);
}

#[test]
fn thinking_scrolls_as_chat_transcript_not_tool_panel() {
    assert!(!is_tool_activity(&MessageType::Thinking(
        "full thoughts".into()
    )));
}
