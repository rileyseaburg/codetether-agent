//! Factory selection and process-wide cache supplied by the application.
use crate::ProviderRegistry;
use std::sync::Arc;
use tokio::sync::OnceCell;
/// Bind provider construction and cache identity to the native application.
/// # Implementors
/// Applications select their existing factory and return a static cache cell.
pub trait Application: Send + Sync + Sized + 'static {
    /// Native factory with the existing task-specific platform fallback.
    type Factory: codetether_provider_registry::task::Host;
    /// Return the process-wide cache used for Vault-backed registries.
    fn cache() -> &'static OnceCell<Arc<ProviderRegistry<Self>>>;
}
