//! Presentation of managed-agent tool outcomes in the TUI.

use crate::tool::ToolResult;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(crate) fn present(app: &mut App, result: &ToolResult, success: String, failure: &str) {
    let kind = if result.success {
        MessageType::System
    } else {
        MessageType::Error
    };
    app.state
        .messages
        .push(ChatMessage::new(kind, result.output.clone()));
    app.state.status = if result.success {
        success
    } else {
        format!("{failure}: {}", result.output)
    };
    app.state.scroll_to_bottom();
}
