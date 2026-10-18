//! Recall presentation using native chat state.
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
};
impl codetether_cognition_core::palace::command::View for App {
    fn push_system_message(&mut self, text: String) {
        self.state
            .messages
            .push(ChatMessage::new(MessageType::System, text));
    }
    fn scroll_to_bottom(&mut self) {
        self.state.scroll_to_bottom();
    }
    fn set_status(&mut self, text: String) {
        self.state.status = text;
    }
}
