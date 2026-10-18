//! Native model resolution and ephemeral output for side questions.
use crate::provider::{Provider, ProviderRegistry};
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
};
use std::sync::Arc;
impl codetether_side_question::Registry for ProviderRegistry {
    fn resolve(registry: &Arc<Self>, model: &str) -> Option<(Arc<dyn Provider>, String)> {
        crate::autochat::model_rotation::resolve_provider_for_model_autochat(registry, model)
    }
}
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
