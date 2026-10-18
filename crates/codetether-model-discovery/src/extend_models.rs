//! Validate model identifiers and expand selectable variants.
use crate::{ModelRefreshSummary, reasoning_refs, selection_policy};
use codetether_provider_core::ModelInfo;
pub(crate) fn extend(
    models: &mut Vec<String>,
    summary: &mut ModelRefreshSummary,
    provider_name: &str,
    provider_models: Vec<ModelInfo>,
) {
    let before = models.len();
    models.extend(provider_models.iter().flat_map(|model| {
        selection_policy::model_ref(&provider_name, model)
            .into_iter()
            .flat_map(|model_ref| reasoning_refs::expand(&provider_name, model_ref))
    }));
    if models.len() > before {
        summary.loaded_providers += 1;
    }
}
