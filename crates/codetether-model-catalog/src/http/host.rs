//! Application provider loading contract for model discovery.
use crate::discovery::CatalogRegistry;
use std::{future::Future, sync::Arc};

/// Load the provider registry used by discovery requests.
/// # Implementors
/// Native applications retain their own credential loading and registry cache.
pub trait Host: Send + Sync + 'static {
    /// Registry supplying provider names and instances for model collection.
    type Registry: CatalogRegistry + Send + Sync + 'static;
    /// Load or reuse the application's provider registry.
    fn load_registry() -> impl Future<Output = anyhow::Result<Arc<Self::Registry>>> + Send;
}
