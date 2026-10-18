//! Native chat view bindings for interactive session forks.
use crate::tui::{app::state::App, chat::message::ChatMessage, models::ViewMode};
pub(super) struct CommandView<'a>(pub(super) &'a mut App);
impl codetether_session_fork::interactive::View for CommandView<'_> {
    fn processing(&self) -> bool {
        self.0.state.processing
    }
    fn messages(&self) -> &[ChatMessage] {
        &self.0.state.messages
    }
    fn set_messages(&mut self, messages: Vec<ChatMessage>) {
        self.0.state.messages = messages;
    }
    fn set_session_id(&mut self, id: String) {
        self.0.state.session_id = Some(id);
    }
    fn clear_streaming_text(&mut self) {
        self.0.state.clear_streaming_text();
    }
    fn clear_request_timing(&mut self) {
        self.0.state.clear_request_timing();
    }
    fn scroll_to_bottom(&mut self) {
        self.0.state.scroll_to_bottom();
    }
    fn show_chat(&mut self) {
        self.0.state.set_view_mode(ViewMode::Chat);
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn push_system_message(&mut self, message: String) {
        super::push_system_message(self.0, message);
    }
}
