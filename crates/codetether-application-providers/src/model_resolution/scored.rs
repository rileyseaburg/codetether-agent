//! Bind score-aware RLM model choices to the application's shared providers.
use crate::{Application, ProviderRegistry};
use codetether_provider_core::Provider;
use codetether_rlm::{RlmConfig, RlmModelPurpose};
use std::sync::Arc;
/// Resolve an RLM model using optional host scoring signals.
/// # Arguments
/// Caller values provide fallbacks; `config`, `purpose` and `score` select the model.
/// # Returns
/// The selected provider and model, retaining the caller on resolution failure.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_providers::Application>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, config: &codetether_rlm::RlmConfig, purpose: codetether_rlm::RlmModelPurpose) {
/// let _selected = codetether_application_providers::model_resolution::resolve_with_score::<H>(provider, "model", config, purpose, |_| None).await;
/// # }
/// ```
pub async fn resolve_with_score<H: Application>(
    caller_provider: Arc<dyn Provider>,
    caller_model: &str,
    config: &RlmConfig,
    purpose: RlmModelPurpose,
    score: impl Fn(&str) -> Option<f64>,
) -> (Arc<dyn Provider>, String) {
    codetether_rlm::model_resolution::resolve_with_score(
        caller_provider,
        caller_model,
        config,
        purpose,
        score,
        ProviderRegistry::<H>::shared_from_vault,
    )
    .await
}
