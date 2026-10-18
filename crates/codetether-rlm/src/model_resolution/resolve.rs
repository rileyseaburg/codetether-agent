//! Resolve a configured model without host scoring.
use super::Registry;
use super::resolve_with_score;
use crate::{RlmConfig, RlmModelPurpose};
use codetether_provider_core::Provider;
use std::{future::Future, sync::Arc};

/// Resolve the configured RLM model for a call surface.
/// # Arguments
/// * `caller_provider`, `caller_model` — Fallback provider and model.
/// * `config`, `purpose` — RLM model-selection settings and call surface.
/// * `loader` — Deferred registry loader for a different selected model.
/// # Returns
/// The resolved provider/model or the original caller pair after any lookup failure.
/// # Examples
/// ```no_run
/// # async fn select<R, F>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, loader: impl FnOnce() -> F)
/// # where R: codetether_rlm::model_resolution::Registry, F: std::future::Future<Output = anyhow::Result<std::sync::Arc<R>>> {
/// let _selected = codetether_rlm::model_resolution::resolve(provider, "caller", &Default::default(), codetether_rlm::RlmModelPurpose::Compression, loader).await;
/// # }
/// ```
pub async fn resolve<R, F>(
    caller_provider: Arc<dyn Provider>,
    caller_model: &str,
    config: &RlmConfig,
    purpose: RlmModelPurpose,
    loader: impl FnOnce() -> F,
) -> (Arc<dyn Provider>, String)
where
    R: Registry,
    F: Future<Output = anyhow::Result<Arc<R>>>,
{
    resolve_with_score(
        caller_provider,
        caller_model,
        config,
        purpose,
        |_| None,
        loader,
    )
    .await
}
