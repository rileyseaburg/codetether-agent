//! Rank capable alternative models and fall back to the caller model.
use codetether_provider_core::ModelInfo;

pub(super) fn select<'a>(models: &'a [ModelInfo], caller_model: &str) -> Option<&'a str> {
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
