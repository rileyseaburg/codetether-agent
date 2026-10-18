//! Native chat state and notice bindings for undo commands.
use crate::tui::{app::state::App, chat::message::ChatMessage};
pub(super) struct CommandView<'a>(pub(super) &'a mut App);
impl codetether_turn_undo::command::View for CommandView<'_> {
    fn processing(&self) -> bool {
        self.0.state.processing
    }
    fn messages(&mut self) -> &mut Vec<ChatMessage> {
        &mut self.0.state.messages
    }
    fn clear_streaming_text(&mut self) {
        self.0.state.clear_streaming_text();
    }
    fn scroll_to_bottom(&mut self) {
        self.0.state.scroll_to_bottom();
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn push_system_message(&mut self, message: String) {
        super::push_system_message(self.0, message);
    }
}
