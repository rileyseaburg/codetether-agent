//! Provider selection without ownership of application initialization.
use anyhow::Result;
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Resolve one configured model while retaining its registry.
/// # Implementors
/// Application provider registries use their existing model resolution.
pub trait Providers {
    /// Resolve a provider and concrete model identifier.
    fn resolve_model(&self, model: &str) -> Result<(Arc<dyn Provider>, String)>;
}
