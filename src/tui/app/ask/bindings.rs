//! Native model resolution and ephemeral output for side questions.
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
};
pub(super) struct Output<'a>(pub(super) &'a mut App);
impl codetether_side_question::View for Output<'_> {
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn push_system_message(&mut self, message: String) {
        self.0
            .state
            .messages
            .push(ChatMessage::new(MessageType::System, message));
    }
    fn scroll_to_bottom(&mut self) {
        self.0.state.scroll_to_bottom();
    }
}
