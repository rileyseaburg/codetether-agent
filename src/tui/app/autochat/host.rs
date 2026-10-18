//! Bind shared autochat execution and presentation to the application.
use super::state::AutochatState;
use crate::provider::{Provider, ProviderRegistry};
use crate::tui::{
    app::state::AppState,
    chat::message::{ChatMessage, MessageType},
};
use codetether_autochat_core::interactive::host::{Host, Registry, View};
use std::sync::Arc;
pub(crate) struct Runtime;
pub(crate) struct NativeRegistry(Arc<ProviderRegistry>);
impl Registry for NativeRegistry {
    fn resolve_model(&self, model: &str) -> anyhow::Result<(Arc<dyn Provider>, String)> {
        self.0.resolve_model(model)
    }
}
impl Host for Runtime {
    type Registry = NativeRegistry;
    async fn load_registry() -> anyhow::Result<Self::Registry> {
        ProviderRegistry::shared_from_vault()
            .await
            .map(NativeRegistry)
    }
}
impl View<crate::session::SessionEvent> for AppState {
    fn autochat(&mut self) -> &mut AutochatState {
        &mut self.autochat
    }
    fn last_completion_model(&self) -> Option<String> {
        self.last_completion_model.clone()
    }
    fn set_status(&mut self, status: String) {
        self.status = status;
    }
    fn push_system(&mut self, text: String) {
        self.messages
            .push(ChatMessage::new(MessageType::System, text));
    }
    fn scroll_to_bottom(&mut self) {
        AppState::scroll_to_bottom(self);
    }
}
