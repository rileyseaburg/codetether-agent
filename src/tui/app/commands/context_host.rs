//! Bind shared context reports to native token estimates and chat.
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
};
impl codetether_context_health::command::View for App {
    fn context(
        &self,
    ) -> (
        &codetether_context_health::ContextHealthState,
        Option<usize>,
        Option<usize>,
    ) {
        (
            &self.state.context_health,
            self.state.context_used,
            self.state.context_budget,
        )
    }
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
