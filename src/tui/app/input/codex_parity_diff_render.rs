use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(super) fn fallback(text: &str) -> &str {
    if text.trim().is_empty() {
        "(none)"
    } else {
        text
    }
}

pub(super) fn finish(app: &mut App, text: String) {
    app.state.clear_input();
    app.state.status = "Diff shown".to_string();
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, text));
    app.state.scroll_to_bottom();
}
