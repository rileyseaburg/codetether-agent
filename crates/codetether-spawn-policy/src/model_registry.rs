//! Provider handle lookup for spawn model eligibility.
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Existing providers whose model catalog can be inspected by spawn policy.
/// # Implementors
/// Applications forward [Self::get] to their initialized provider registry.
pub trait ModelRegistry: Sync {
    /// Clone an existing provider handle by name; return none if unavailable.
    fn get(&self, provider: &str) -> Option<Arc<dyn Provider>>;
}
