//! Render a single tool-activity item (call / result / thinking).

use ratatui::text::Line;

use super::item_call::render_tool_call;
use super::item_result::render_tool_result;
use super::item_thinking::render_thinking;
use codetether_chat_model::{ChatMessage, MessageType};
use codetether_chat_status::format_timestamp;

pub(super) fn render_tool_activity_item(
    body_lines: &mut Vec<Line<'static>>,
    message: &ChatMessage,
    preview_width: usize,
) {
    let timestamp = format_timestamp(message.timestamp);
    match &message.message_type {
        MessageType::ToolCall { name, arguments } => {
            render_tool_call(body_lines, &timestamp, name, arguments, preview_width)
        }
        MessageType::ToolResult {
            name,
            output,
            success,
            duration_ms,
        } => render_tool_result(
            body_lines,
            &timestamp,
            name,
            output,
            *success,
            *duration_ms,
            preview_width,
        ),
        MessageType::Thinking(thoughts) => {
            render_thinking(body_lines, &timestamp, thoughts, preview_width)
        }
        _ => {}
    }
}
