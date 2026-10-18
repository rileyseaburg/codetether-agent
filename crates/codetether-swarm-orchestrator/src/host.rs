//! Provider registry operations supplied by the application.
use codetether_provider_core::Provider;
use std::{future::Future, sync::Arc};
/// Load and query providers without depending on native session/runtime code.
/// # Implementors
/// Implement using the application's existing Vault-backed provider registry.
pub trait Host: Send + Sync + 'static {
    /// Registry kept for the lifetime of the orchestrator.
    type Providers: Send + Sync;
    /// Load the provider registry.
    /// # Errors
    /// Returns provider initialization errors.
    fn load() -> impl Future<Output = anyhow::Result<Self::Providers>> + Send;
    /// List registered provider names in their existing order.
    fn list(providers: &Self::Providers) -> Vec<&str>;
    /// Retrieve one provider by its canonical name.
    fn get(providers: &Self::Providers, name: &str) -> Option<Arc<dyn Provider>>;
}
