//! Host-side provider resolution for crate-owned RLM model choices.

use std::sync::Arc;

use crate::provider::{Provider, ProviderRegistry};
use crate::rlm::{RlmConfig, RlmModelPurpose};
#[path = "rlm_model/registry.rs"]

/// Resolve the configured RLM model for a call surface.
pub(crate) async fn resolve(
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
        ProviderRegistry::shared_from_vault,
    )
    .await
}

/// Resolve an RLM model using optional host scoring signals.
pub(crate) async fn resolve_with_score(
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
        ProviderRegistry::shared_from_vault,
    )
    .await
}
