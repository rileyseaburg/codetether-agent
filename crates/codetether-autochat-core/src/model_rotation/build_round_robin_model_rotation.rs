//! Auto-chat build round robin model rotation.
use super::*;

/// Build a rotation with the requested model first and at most one entry per provider.
///
/// # Arguments
/// * `registry` - The existing initialized provider registry.
/// * `requested_model_ref` - Requested model reference.
///
/// # Returns
/// A rotation retaining provider priority and model fallback rules.
///
/// # Examples
/// ```no_run
/// # async fn example(registry: &dyn codetether_autochat_core::model_rotation::ModelRegistry) {
/// use codetether_autochat_core::model_rotation::build_round_robin_model_rotation;
/// let rotation = build_round_robin_model_rotation(registry, "provider/model").await;
/// # }
/// ```
pub async fn build_round_robin_model_rotation(
    registry: &dyn ModelRegistry,
    requested_model_ref: &str,
) -> RelayModelRotation {
    let mut rotation = RelayModelRotation::default();
    let mut used_providers = HashSet::<String>::new();

    if let Some(model_ref) = normalize_requested_model_ref(registry, requested_model_ref) {
        add_unique_model_ref(&mut rotation.model_refs, &mut used_providers, &model_ref);
    }

    for provider_name in PRIORITY_PROVIDERS {
        if let Some(model_ref) = preferred_model_ref_for_provider(registry, provider_name).await {
            add_unique_model_ref(&mut rotation.model_refs, &mut used_providers, &model_ref);
        }
    }

    if rotation.model_refs.is_empty() {
        rotation.model_refs.push(requested_model_ref.to_string());
    }

    rotation
}
