use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

const PROMPT: &str = "Review the current uncommitted changes for bugs, regressions, security issues, and missing tests. Lead with findings and cite file:line references.";

pub(super) fn prepare(app: &mut App) {
    app.state.clear_input();
    app.state.input = PROMPT.to_string();
    app.state.input_cursor = app.state.input.chars().count();
    app.state.status = "Review prompt ready".to_string();
    app.state.messages.push(ChatMessage::new(
        MessageType::System,
        "Review prompt is ready. Press Enter to run it.",
    ));
    app.state.scroll_to_bottom();
}
