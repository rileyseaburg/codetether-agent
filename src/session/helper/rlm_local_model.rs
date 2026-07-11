//! Same-provider model selection for budget-critical RLM work.

use std::sync::Arc;

use crate::provider::{ModelInfo, Provider};

/// Choose an RLM model exposed by the caller's provider.
pub(super) async fn resolve(
    provider: Arc<dyn Provider>,
    caller_model: &str,
) -> (Arc<dyn Provider>, String) {
    let Ok(models) = provider.list_models().await else {
        return (provider, caller_model.to_string());
    };
    let model = select(&models, caller_model).unwrap_or(caller_model);
    tracing::info!(
        provider = provider.name(),
        rlm_model = model,
        "Selected same-provider RLM model"
    );
    (provider, model.to_string())
}

fn select<'a>(models: &'a [ModelInfo], caller_model: &str) -> Option<&'a str> {
    models
        .iter()
        .filter(|model| model.supports_tools && model.id != caller_model)
        .min_by(|left, right| rank(left).total_cmp(&rank(right)))
        .or_else(|| models.iter().find(|model| model.id == caller_model))
        .map(|model| model.id.as_str())
}

fn rank(model: &ModelInfo) -> f64 {
    model.input_cost_per_million.unwrap_or(f64::MAX / 2.0)
        + model.context_window as f64 / 1_000_000_000.0
}

#[cfg(test)]
#[path = "rlm_local_model_tests.rs"]
mod tests;
