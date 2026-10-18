//! Register model-backed mutable tools.
use crate::Host;
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Register model-backed mutable tools.
///
/// # Arguments
/// * `registry` — Registry receiving RLM and Ralph tools.
/// * `provider` — Shared provider, cloned for RLM and transferred to Ralph.
/// * `model` — Resolved model, cloned for RLM and transferred to Ralph.
/// * `H` — Application registry and constructor bindings.
/// # Returns
/// Registers this group in the supplied registry.
/// # Examples
/// See [crate] for a generic application-bound construction example.
pub fn register_model_tools<H: Host>(
    registry: &mut H::Registry,
    provider: Arc<dyn Provider>,
    model: String,
) {
    H::register(registry, H::rlm(Arc::clone(&provider), model.clone()));
    H::register(registry, H::ralph(provider, model));
}
