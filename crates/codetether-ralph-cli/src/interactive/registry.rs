//! Provider lookup required by interactive Ralph run selection.
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Existing provider registry operations used by the interactive command.
/// # Implementors
/// Native registries delegate to their current model-selection policy.
pub trait Registry {
    /// Resolve the selected model to its provider and provider-local name.
    /// # Errors
    /// Returns the existing model resolution failure.
    fn resolve_model(&self, model: &str) -> anyhow::Result<(Arc<dyn Provider>, String)>;
}
