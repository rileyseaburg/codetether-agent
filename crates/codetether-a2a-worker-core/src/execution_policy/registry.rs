//! Provider lookup shared by worker model selection and session execution.
use codetether_provider_core::Provider;
use std::sync::Arc;

/// Provider lookup used by worker selection policy.
/// # Implementors
/// Configured provider registries retain ordering and lookup semantics.
pub trait Registry: Send + Sync {
    /// Configured provider names.
    fn list(&self) -> Vec<&str>;
    /// Resolve a provider client by name.
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>>;
}
