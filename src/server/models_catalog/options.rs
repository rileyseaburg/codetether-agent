//! Per-model thinking-effort and service-tier options for discovery.
//!
//! Lets API clients find which `reasoning_effort` and `service_tier`
//! values `/v1/chat/completions` accepts for each model.

use serde::Serialize;

/// Request options a model accepts beyond the base OpenAI fields.
#[derive(Debug, Default, Serialize)]
pub(crate) struct ModelOptions {
    /// Accepted `reasoning_effort` values; empty when unsupported.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) reasoning_efforts: Vec<String>,
    /// Accepted `service_tier` values such as `fast` and `ultrafast`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) service_tiers: Vec<String>,
}

/// Look up options for `model` served by `provider`.
pub(crate) fn options(provider: &str, model: &str) -> ModelOptions {
    if provider != "openai-codex" {
        return ModelOptions::default();
    }
    use crate::provider::openai_codex::{reasoning_catalog, service_tier_catalog};
    ModelOptions {
        reasoning_efforts: reasoning_catalog::supported_levels(model)
            .into_iter()
            .map(str::to_string)
            .collect(),
        service_tiers: service_tier_catalog::suffixes(model)
            .into_iter()
            .map(|suffix| suffix.trim_start_matches('-').to_string())
            .collect(),
    }
}
