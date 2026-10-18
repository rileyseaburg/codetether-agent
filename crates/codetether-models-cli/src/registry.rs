//! Minimal access to providers already loaded by the native frontend.
use codetether_provider_core::Provider;
use std::sync::Arc;

/// Supplies configured providers without owning authentication or credentials.
///
/// # Implementors
/// Frontends delegate these methods to their initialized provider registry.
pub trait Registry: Sync {
    /// Return the names of the currently configured providers.
    fn list(&self) -> Vec<&str>;
    /// Look up a configured provider by its registry name.
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>>;
}
