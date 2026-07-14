//! TUI state updates for completed mux commands.

use anyhow::Result;

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(super) fn show(app: &mut App, result: Result<String>) {
    let (kind, content, status) = match result {
        Ok(content) => (MessageType::System, content, "Mux command completed"),
        Err(error) => (
            MessageType::Error,
            format!("Mux error: {error}"),
            "Mux command failed",
        ),
    };
    app.state.messages.push(ChatMessage::new(kind, content));
    app.state.status = status.into();
    app.state.scroll_to_bottom();
}
