//! Resolve scored model choices while retaining caller fallback.
use super::Registry;
use crate::{RlmConfig, RlmModelPurpose, select_rlm_model};
use codetether_provider_core::Provider;
use std::{future::Future, sync::Arc};

/// Resolve an RLM model using optional host scoring signals.
/// # Arguments
/// * `caller_provider`, `caller_model` — Fallback provider and model.
/// * `config`, `purpose` — RLM model-selection settings and call surface.
/// * `score` — Optional score for candidate model references.
/// * `loader` — Deferred registry loader for a different selected model.
/// # Returns
/// The resolved provider/model or the original caller pair after any lookup failure.
/// # Examples
/// ```no_run
/// # async fn select<R, F>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, loader: impl FnOnce() -> F)
/// # where R: codetether_rlm::model_resolution::Registry, F: std::future::Future<Output = anyhow::Result<std::sync::Arc<R>>> {
/// let _selected = codetether_rlm::model_resolution::resolve_with_score(provider, "caller", &Default::default(), codetether_rlm::RlmModelPurpose::Compression, |_| None, loader).await;
/// # }
/// ```
pub async fn resolve_with_score<R, F>(
    caller_provider: Arc<dyn Provider>,
    caller_model: &str,
    config: &RlmConfig,
    purpose: RlmModelPurpose,
    score: impl Fn(&str) -> Option<f64>,
    loader: impl FnOnce() -> F,
) -> (Arc<dyn Provider>, String)
where
    R: Registry,
    F: Future<Output = anyhow::Result<Arc<R>>>,
{
    let choice = select_rlm_model(config, purpose, caller_model, score);
    if choice.model == caller_model {
        return (caller_provider, caller_model.to_string());
    }
    let Ok(registry) = loader().await else {
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
