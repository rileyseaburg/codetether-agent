//! Bind configured RLM model choices to the application's shared provider cache.
use crate::{Application, ProviderRegistry};
use codetether_provider_core::Provider;
use codetether_rlm::{RlmConfig, RlmModelPurpose};
use std::sync::Arc;
/// Resolve the configured RLM model for a call surface.
/// # Arguments
/// `caller_provider` and `caller_model` are fallbacks; `config` and `purpose` select policy.
/// # Returns
/// The selected provider and model, retaining the caller when resolution fails.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_providers::Application>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, config: &codetether_rlm::RlmConfig, purpose: codetether_rlm::RlmModelPurpose) {
/// let _selected = codetether_application_providers::model_resolution::resolve::<H>(provider, "model", config, purpose).await;
/// # }
/// ```
pub async fn resolve<H: Application>(
    caller_provider: Arc<dyn Provider>,
    caller_model: &str,
    config: &RlmConfig,
    purpose: RlmModelPurpose,
) -> (Arc<dyn Provider>, String) {
    codetether_rlm::model_resolution::resolve(
        caller_provider,
        caller_model,
        config,
        purpose,
        ProviderRegistry::<H>::shared_from_vault,
    )
    .await
}
