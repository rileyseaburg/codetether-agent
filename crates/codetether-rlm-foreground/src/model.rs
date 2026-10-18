//! Resolve a same-provider model while preserving caller fallback.
use super::selection::select;
use codetether_provider_core::Provider;
use std::sync::Arc;

/// Choose an RLM model exposed by the caller's provider.
pub(super) async fn resolve(
    provider: Arc<dyn Provider>,
    caller_model: &str,
) -> (Arc<dyn Provider>, String) {
    let Ok(models) = provider.list_models().await else {
        return (provider, caller_model.to_string());
    };
    let model = select(&models, caller_model).unwrap_or(caller_model);
    tracing::info!(target: "codetether_agent::session::helper::compression::compression_defer::local_model",
        provider = provider.name(),
        rlm_model = model,
        "Selected same-provider RLM model"
    );
    (provider, model.to_string())
}
