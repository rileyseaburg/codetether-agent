//! Provider loading and presentation interfaces for interactive autochat.
use super::state::AutochatState;
use codetether_provider_core::Provider;
use std::{future::Future, sync::Arc};
/// Resolves a model after the provider registry is loaded.
pub trait Registry: Send {
    /// Resolve the model reference.
    ///
    /// # Errors
    /// Returns the registry error when the requested model is unavailable.
    fn resolve_model(&self, model: &str) -> anyhow::Result<(Arc<dyn Provider>, String)>;
}
/// Supplies application provider loading for the relay.
pub trait Host {
    /// Loaded provider registry.
    type Registry: Registry;
    /// Load provider configuration; errors become relay completion events.
    ///
    /// # Errors
    /// Returns provider configuration or initialization failures.
    fn load_registry() -> impl Future<Output = anyhow::Result<Self::Registry>> + Send;
}
/// Supplies application state and presentation for relay commands and events.
pub trait View<E> {
    /// Borrow relay state.
    fn autochat(&mut self) -> &mut AutochatState<E>;
    /// Read the most recently completed model.
    fn last_completion_model(&self) -> Option<String>;
    /// Replace the status line.
    fn set_status(&mut self, status: String);
    /// Append a system message.
    fn push_system(&mut self, text: String);
    /// Scroll chat to the latest message.
    fn scroll_to_bottom(&mut self);
}
