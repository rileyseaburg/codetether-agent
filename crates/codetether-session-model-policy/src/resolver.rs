//! Synchronous model resolution supplied by the application registry.
use anyhow::Result;
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Resolves configured selectors to a provider and provider-local model name.
/// # Implementors
/// Implement with the existing registry's model lookup operation.
pub trait ModelResolver {
    /// Resolve one configured model selector.
    /// # Errors
    /// Returns the registry's lookup error when the selector cannot be resolved.
    fn resolve_model(&self, selector: &str) -> Result<(Arc<dyn Provider>, String)>;
}
