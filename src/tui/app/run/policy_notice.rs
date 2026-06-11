use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(super) fn push(app: &mut App, policy: &str) {
    let content = format!("Execution policy: {policy}");
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, content));
    app.state.scroll_to_bottom();
}
