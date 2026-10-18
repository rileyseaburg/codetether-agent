//! Task-scoped provider selection with platform fallback.
//!
//! Supply [Host] for the platform cache and use [load_task_provider_registry]
//! with the optional provider-key claim. Malformed claims preserve fallback behavior.
mod selection;
use crate::ProviderRegistry;
pub use selection::load_task_provider_registry;
/// Platform cache access paired with the shared native provider constructors.
pub trait Host: codetether_provider_factory::Host {
    /// Clone the cached platform registry.
    /// # Errors
    /// Returns platform registry loading failures before task-specific context is added.
    fn platform_registry()
    -> impl std::future::Future<Output = anyhow::Result<ProviderRegistry>> + Send;
}
