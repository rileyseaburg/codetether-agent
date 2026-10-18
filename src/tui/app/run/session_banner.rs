//! Apply shared session-outcome banner text to the chat transcript.
use super::session_outcome::SessionLoadOutcome;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
pub(super) fn push(outcome: &SessionLoadOutcome, app: &mut App) {
    if let Some(message) = outcome.banner() {
        app.state
            .messages
            .push(ChatMessage::new(MessageType::System, message));
        app.state.scroll_to_bottom();
    }
}
