//! Provider lookup and ephemeral presentation operations.
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Existing provider selection policy for side questions.
/// # Implementors
/// Applications delegate to their current relay/provider model resolver.
pub trait Registry: Sized {
    /// Resolve a provider and provider-local model without changing the registry.
    fn resolve(registry: &Arc<Self>, model: &str) -> Option<(Arc<dyn Provider>, String)>;
}
/// Status and system-message output for an ephemeral answer.
/// # Implementors
/// Interactive clients provide presentation access without session mutation.
pub trait View {
    /// Replace the current status message.
    fn set_status(&mut self, status: String);
    /// Append the side answer as a system message.
    fn push_system_message(&mut self, message: String);
    /// Scroll to the displayed answer.
    fn scroll_to_bottom(&mut self);
}
