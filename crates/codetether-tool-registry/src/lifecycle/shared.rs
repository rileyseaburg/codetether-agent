use super::Host;
use codetether_provider_core::Provider;
use std::sync::Arc;

/// Build a default shared registry with batch support.
/// # Arguments
/// Native factories are selected by the `H` type parameter.
/// # Returns
/// The populated registry; batch holds a weak reference to this same instance.
/// # Examples
/// ```no_run
/// fn build<H: codetether_tool_registry::lifecycle::Host>() -> std::sync::Arc<H::Registry> {
///     codetether_tool_registry::lifecycle::with_defaults::<H>()
/// }
/// ```
pub fn with_defaults<H: Host>() -> Arc<H::Registry> {
    let mut registry = H::defaults();
    let batch = H::batch();
    H::register_batch(&mut registry, Arc::clone(&batch));
    finish::<H>(registry, batch)
}

/// Build a provider-backed shared registry with batch and routed search.
/// # Arguments
/// * `provider` — Provider shared by catalog tools and routed search.
/// * `model` — Model selected for the provider-backed catalog.
/// # Returns
/// The populated registry, bound weakly to its batch tool after search is added.
/// # Examples
/// ```no_run
/// fn build<H: codetether_tool_registry::lifecycle::Host>(
///     provider: std::sync::Arc<dyn codetether_provider_core::Provider>, model: String,
/// ) -> std::sync::Arc<H::Registry> {
///     codetether_tool_registry::lifecycle::with_provider::<H>(provider, model)
/// }
/// ```
pub fn with_provider<H: Host>(provider: Arc<dyn Provider>, model: String) -> Arc<H::Registry> {
    let mut registry = H::provider(Arc::clone(&provider), model);
    let batch = H::batch();
    H::register_batch(&mut registry, Arc::clone(&batch));
    H::register_search(&mut registry, provider);
    finish::<H>(registry, batch)
}

fn finish<H: Host>(registry: H::Registry, batch: Arc<H::Batch>) -> Arc<H::Registry> {
    let registry = Arc::new(registry);
    H::bind_batch(&batch, Arc::downgrade(&registry));
    registry
}
