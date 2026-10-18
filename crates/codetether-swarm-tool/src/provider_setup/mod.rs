//! Resolve a direct-swarm provider from an asynchronously loaded registry.
//!
//! Implement [Registry] for the native provider collection, then pass its load
//! future to [load]. Provider failures retain their original observer and tool
//! reporting semantics; caching and credentials remain owned by the loader.
mod load;
use codetether_provider_core::Provider;
pub use load::load;
use std::sync::Arc;
/// Read-only provider lookup required during direct-swarm setup.
/// # Implementors
/// The application provider registry supplies names and existing provider Arcs.
pub trait Registry: Send + Sync {
    /// List registered provider names without reordering them.
    fn list(&self) -> Vec<&str>;
    /// Clone the registered provider with the requested identifier.
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>>;
}
