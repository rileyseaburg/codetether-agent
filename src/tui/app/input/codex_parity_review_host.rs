//! Native composer and transcript bindings for review preparation.
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
};
impl codetether_slash_command::utility::review::View for App {
    fn clear_input(&mut self) {
        self.state.clear_input();
    }
    fn input(&mut self) -> &mut String {
        &mut self.state.input
    }
    fn cursor(&mut self) -> &mut usize {
        &mut self.state.input_cursor
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
    fn push_system(&mut self, text: String) {
        self.state
            .messages
            .push(ChatMessage::new(MessageType::System, text));
    }
    fn scroll_to_bottom(&mut self) {
        self.state.scroll_to_bottom();
    }
}
