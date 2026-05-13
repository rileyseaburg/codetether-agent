//! Host-side provider resolution for crate-owned RLM model choices.

use std::sync::Arc;

use crate::provider::{Provider, ProviderRegistry};
use crate::rlm::{RlmConfig, RlmModelPurpose, select_rlm_model};

/// Resolve the configured RLM model for a call surface.
pub(crate) async fn resolve(
    caller_provider: Arc<dyn Provider>,
    caller_model: &str,
    config: &RlmConfig,
    purpose: RlmModelPurpose,
) -> (Arc<dyn Provider>, String) {
    resolve_with_score(caller_provider, caller_model, config, purpose, |_| None).await
}

/// Resolve an RLM model using optional host scoring signals.
pub(crate) async fn resolve_with_score(
    caller_provider: Arc<dyn Provider>,
    caller_model: &str,
    config: &RlmConfig,
    purpose: RlmModelPurpose,
    score: impl Fn(&str) -> Option<f64>,
) -> (Arc<dyn Provider>, String) {
    let choice = select_rlm_model(config, purpose, caller_model, score);
    if choice.model == caller_model {
        return (caller_provider, caller_model.to_string());
    }
    let Ok(registry) = ProviderRegistry::shared_from_vault().await else {
        tracing::warn!("RLM: provider registry unavailable; falling back to caller model");
        return (caller_provider, caller_model.to_string());
    };
    match registry.resolve_model(&choice.model) {
        Ok((provider, model)) => {
            tracing::info!(
                rlm_model = %model,
                source = ?choice.source,
                "RLM: selected dedicated model"
            );
            (provider, model)
        }
        Err(error) => {
            tracing::warn!(
                rlm_model = %choice.model,
                error = %error,
                "RLM: model resolution failed; falling back to caller model"
            );
            (caller_provider, caller_model.to_string())
        }
    }
}
