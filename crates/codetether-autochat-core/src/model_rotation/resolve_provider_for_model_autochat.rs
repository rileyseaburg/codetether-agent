//! Auto-chat resolve provider for model autochat.
use super::*;

/// Resolve a provider/model reference using aliases and configured fallback order.
///
/// # Arguments
/// * `registry` - The existing initialized provider registry.
/// * `model_ref` - Requested model reference.
///
/// # Returns
/// The existing provider handle and selected model name, if a provider is available.
///
/// # Examples
/// ```no_run
/// # fn example(registry: &dyn codetether_autochat_core::model_rotation::ModelRegistry) {
/// use codetether_autochat_core::model_rotation::resolve_provider_for_model_autochat;
/// let resolved = resolve_provider_for_model_autochat(registry, "provider/model");
/// # }
/// ```
pub fn resolve_provider_for_model_autochat(
    registry: &dyn ModelRegistry,
    model_ref: &str,
) -> Option<(Arc<dyn Provider>, String)> {
    let (provider_name, model_name) =
        codetether_provider_core::routing::parse_model_string(model_ref);
    if let Some(provider_name) = provider_name {
        let normalized = normalize_provider_name(provider_name);
        return registry
            .get(normalized)
            .map(|provider| (provider, model_name.to_string()));
    }

    for provider_name in RESOLUTION_FALLBACKS {
        if let Some(provider) = registry.get(provider_name) {
            return Some((provider, model_ref.to_string()));
        }
    }

    registry
        .list()
        .first()
        .copied()
        .and_then(|name| registry.get(name))
        .map(|provider| (provider, model_ref.to_string()))
}
