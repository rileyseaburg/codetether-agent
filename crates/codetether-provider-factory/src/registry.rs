//! Registration operations supplied by the application registry.
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Provider collection with application-controlled instrumentation.
/// Implement this to retain native wrapping when factory policy registers a provider.
pub trait Registry {
    /// Whether the exact provider key is already registered.
    fn contains(&self, name: &str) -> bool;
    /// Register a provider using the application's instrumentation policy.
    fn register(&mut self, provider: Arc<dyn Provider>);
    /// Number of registered provider keys.
    fn len(&self) -> usize;
    /// Whether the provider collection is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
