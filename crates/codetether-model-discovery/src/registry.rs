//! Provider lookup supplied by the host application.
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Exposes configured provider instances to catalog discovery.
/// # Implementors
/// Applications forward these operations to their provider registry.
pub trait Registry: Sync {
    /// Return configured provider names in registry order.
    fn list(&self) -> Vec<&str>;
    /// Return the shared provider instance for a name, if still available.
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>>;
}
