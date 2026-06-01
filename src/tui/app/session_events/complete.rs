//! Tool-call complete event.

use crate::tui::app::state::App;
use crate::tui::app::text::truncate_preview;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(crate) fn complete(
    app: &mut App,
    name: String,
    output: String,
    success: bool,
    duration_ms: u64,
) {
    app.state.reset_tool_preview_scroll();
    app.state.messages.push(ChatMessage::new(
        MessageType::ToolResult {
            name: name.clone(),
            output: crate::tui::chat::payload::tool_output(&output),
            success,
            duration_ms: Some(duration_ms),
        },
        format!("{name}: {}", truncate_preview(&output, 600)),
    ));
    app.state
        .note_tool_completed(name.clone(), duration_ms, success);
    app.state.status = status_text(&name, success, duration_ms, &output);
    app.state.scroll_to_bottom();
}

fn status_text(name: &str, success: bool, duration_ms: u64, output: &str) -> String {
    let mark = if success { "✓" } else { "✗" };
    let preview = truncate_preview(&output.replace('\n', " "), 80);
    format!("{mark} {name} finished in {duration_ms}ms — {preview}")
}
