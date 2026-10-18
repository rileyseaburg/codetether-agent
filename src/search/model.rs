//! Bind router model resolution to the application's provider registry.
use crate::provider::{Provider, ProviderRegistry};
pub use codetether_search_router::model::DEFAULT_ROUTER_MODEL;
use std::sync::Arc;
/// Resolve a router model using the shared resolver and existing registry.
///
/// # Arguments
/// * `registry` - Configured providers in fallback order.
/// * `model_ref` - Provider/model pair or local model identifier.
/// # Returns
/// The selected provider and its local model identifier.
/// # Errors
/// Returns an error if no suitable provider is available.
/// # Examples
/// ```rust
/// let registry = codetether_agent::provider::ProviderRegistry::new();
/// assert!(codetether_agent::search::model::resolve_router_model(&registry, "model").is_err());
/// ```
pub fn resolve_router_model(
    registry: &ProviderRegistry,
    model_ref: &str,
) -> anyhow::Result<(Arc<dyn Provider>, String)> {
    codetether_search_router::model::resolve_router_model(registry, model_ref)
}
