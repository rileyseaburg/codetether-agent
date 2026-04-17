//! Ctrl+Y handler: copy latest assistant reply to clipboard.

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::clipboard_text::copy_text;
use crate::tui::constants::SCROLL_BOTTOM;

fn clipboard_text(msg: &ChatMessage) -> String {
    match &msg.message_type {
        MessageType::Thinking(text) => text.clone(),
        MessageType::ToolCall {
            name, arguments, ..
        } => {
            format!("Tool call: {name}\n{arguments}")
        }
        MessageType::ToolResult { name, output, .. } => {
            format!("Tool result: {name}\n{output}")
        }
        MessageType::Image { url } => url.clone(),
        MessageType::File { path, .. } => path.clone(),
        _ => msg.content.clone(),
    }
}

pub(super) fn handle_copy_reply(app: &mut App) {
    let msg =
        app.state.messages.iter().rev().find(|m| {
            matches!(m.message_type, MessageType::Assistant) && !m.content.trim().is_empty()
        });
    let Some(msg) = msg else {
        app.state.status = "Nothing to copy yet.".into();
        return;
    };
    let text = clipboard_text(msg);
    match copy_text(&text) {
        Ok(method) => {
            app.state.status = format!("Copied latest reply ({method}).");
            app.state.chat_scroll = SCROLL_BOTTOM;
        }
        Err(err) => {
            tracing::warn!(error = %err, "Copy to clipboard failed");
            app.state.status = "Could not copy to clipboard.".into();
        }
    }
}
