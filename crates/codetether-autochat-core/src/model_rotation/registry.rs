//! Read-only provider lookup needed by the rotation policy.
use super::*;
/// Adapter over an initialized registry; preserves its listing order.
///
/// # Implementors
/// Application registries forward these methods without registering providers
/// or reordering names. Provider handles retain their existing instrumentation.
pub trait ModelRegistry: Send + Sync {
    /// Return the provider registered under this exact name.
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>>;
    /// Return provider names in the registry's existing iteration order.
    fn list(&self) -> Vec<&str>;
}
