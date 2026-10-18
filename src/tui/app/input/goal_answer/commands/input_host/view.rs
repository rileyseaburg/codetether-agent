//! Direct composer and transcript updates for shared input handling.
use super::Native;
use crate::tui::chat::message::{ChatMessage, MessageType};
impl codetether_goal_command::input::View for Native<'_> {
    fn input(&self) -> &str {
        &self.app.state.input
    }
    fn status(&mut self, text: String) {
        self.app.state.status = text;
    }
    fn history(&mut self, text: String) {
        self.app.state.push_history(text);
    }
    fn clear(&mut self) {
        self.app.state.clear_input();
    }
    fn redraw(&mut self) {
        self.app.state.needs_redraw = true;
    }
    fn system(&mut self, text: String) {
        self.app
            .state
            .messages
            .push(ChatMessage::new(MessageType::System, text));
    }
    fn scroll(&mut self) {
        self.app.state.scroll_to_bottom();
    }
}
