//! Auto-chat add unique model ref.
use super::*;

pub(super) fn add_unique_model_ref(
    model_refs: &mut Vec<String>,
    used_providers: &mut HashSet<String>,
    model_ref: &str,
) {
    let (provider_name, _) = codetether_provider_core::routing::parse_model_string(model_ref);
    let Some(provider_name) = provider_name else {
        return;
    };
    if used_providers.insert(provider_name.to_string()) {
        model_refs.push(model_ref.to_string());
    }
}
