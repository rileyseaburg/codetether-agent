//! Native chat presentation for interactive A2A notices.
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
};
impl codetether_a2a_interactive::Presentation for App {
    fn push_system(&mut self, text: String) {
        self.state
            .messages
            .push(ChatMessage::new(MessageType::System, text));
    }
    fn scroll_to_bottom(&mut self) {
        self.state.scroll_to_bottom();
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
}
