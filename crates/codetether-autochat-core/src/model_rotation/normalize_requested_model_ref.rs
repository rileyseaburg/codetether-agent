//! Auto-chat normalize requested model ref.
use super::*;

pub(super) fn normalize_requested_model_ref(
    registry: &dyn ModelRegistry,
    requested_model_ref: &str,
) -> Option<String> {
    let (provider_name, model_name) =
        codetether_provider_core::routing::parse_model_string(requested_model_ref);
    if let Some(provider_name) = provider_name {
        let normalized = normalize_provider_name(provider_name);
        if registry.get(normalized).is_some() {
            return Some(format!("{normalized}/{model_name}"));
        }
    }

    resolve_provider_for_model_autochat(registry, requested_model_ref)
        .map(|(provider, model)| format!("{}/{}", provider.name(), model))
}
