//! Provider lookup interface without moving ownership of provider discovery.
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Read-only provider access for router model selection.
///
/// # Implementors
/// Implement this for the application's existing provider registry. Preserve
/// its listing order, which determines fallback selection.
pub trait Registry: Send + Sync {
    /// Look up an already configured provider by name.
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>>;
    /// List configured providers in fallback-preference order.
    fn list(&self) -> Vec<&str>;
}
