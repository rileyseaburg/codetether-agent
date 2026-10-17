//! Auto-chat preferred model ref for provider.
use super::*;

pub(super) async fn preferred_model_ref_for_provider(
    registry: &dyn ModelRegistry,
    provider_name: &str,
) -> Option<String> {
    let provider = registry.get(provider_name)?;
    let listed = provider.list_models().await.unwrap_or_default();
    if let Some(model_id) = choose_model_from_list(provider_name, &listed) {
        return Some(format!("{provider_name}/{model_id}"));
    }
    default_model_for_provider(provider_name).map(|model_id| format!("{provider_name}/{model_id}"))
}
