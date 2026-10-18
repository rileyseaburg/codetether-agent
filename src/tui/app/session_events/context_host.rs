//! Native health, token and message bindings for context events.
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
};
use codetether_context_health::ContextHealthState;
pub(super) struct Native<'a>(pub &'a mut App);
impl codetether_context_health::events::View for Native<'_> {
    fn health(&mut self) -> &mut ContextHealthState {
        &mut self.0.state.context_health
    }
    fn set_used(&mut self, tokens: usize) {
        self.0.state.context_used = Some(tokens);
    }
    fn set_budget(&mut self, tokens: usize) {
        self.0.state.context_budget = Some(tokens);
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn push_error(&mut self, text: String) {
        self.0
            .state
            .messages
            .push(ChatMessage::new(MessageType::Error, text));
    }
}
