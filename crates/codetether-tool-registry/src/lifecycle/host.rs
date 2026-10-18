use codetether_provider_core::Provider;
use std::sync::{Arc, Weak};

/// Native factories and bindings used during shared registry initialization.
///
/// # Implementors
/// Applications provide their registry and batch types. Construction methods
/// populate the ordinary catalog; registration methods add batch and search.
/// `bind_batch` stores only the supplied weak registry reference.
pub trait Host {
    /// Application registry whose tools may refer back to it weakly.
    type Registry;
    /// Batch tool constructed before the registry becomes shared.
    type Batch;
    /// Construct the default catalog without batch.
    fn defaults() -> Self::Registry;
    /// Construct the provider-backed catalog without batch or routed search.
    fn provider(provider: Arc<dyn Provider>, model: String) -> Self::Registry;
    /// Construct a batch tool without a registry reference.
    fn batch() -> Arc<Self::Batch>;
    /// Register the same batch instance that will receive the weak reference.
    fn register_batch(registry: &mut Self::Registry, batch: Arc<Self::Batch>);
    /// Register routed search using the supplied provider.
    fn register_search(registry: &mut Self::Registry, provider: Arc<dyn Provider>);
    /// Bind batch to the fully populated shared registry without retaining it.
    fn bind_batch(batch: &Self::Batch, registry: Weak<Self::Registry>);
}
