//! Read-only provider lookup used by cloud fallback.
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Access to already-initialized embedding providers.
/// # Implementors
/// Application registries clone the requested provider without changing order.
pub trait Registry {
    /// Get a provider by its registered name.
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>>;
}
