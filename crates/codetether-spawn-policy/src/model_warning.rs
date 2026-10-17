//! Advisory spawn warning for models outside the eligibility policy.
use crate::{ModelRegistry, is_free_or_eligible};
/// Return the existing advisory warning when a model is not eligible.
/// # Arguments
/// * `model` — Requested model reference.
/// * `registry` — Initialized providers available for catalog inspection.
/// # Returns
/// None for eligible models, otherwise the existing metered-cost warning.
/// # Examples
/// ```rust,no_run
/// use codetether_spawn_policy::{model_warning, ModelRegistry};
/// async fn example<R: ModelRegistry>(registry: &R) {
///     let _ = model_warning("provider/model", registry).await;
/// }
/// ```
pub async fn for_model<R: ModelRegistry>(model: &str, registry: &R) -> Option<String> {
    if is_free_or_eligible(model, registry).await {
        return None;
    }
    Some(ineligible_model_warning(model))
}

fn ineligible_model_warning(model: &str) -> String {
    format!(
        "warning: '{model}' is not free/subscription-eligible; \
         this spawn may incur metered costs. Prefer ':free' models, \
         subscription providers, or OAuth providers."
    )
}
