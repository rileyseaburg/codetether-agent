//! Wire format of the Codex backend `/models` response.
//!
//! Only the fields CodeTether uses are decoded; unknown fields are ignored so
//! upstream schema growth never breaks discovery.

use serde::Deserialize;
#[path = "model_discovery_capabilities.rs"]
mod capabilities;
use capabilities::{ReasoningEffort, ServiceTier};

/// One model entry as published by the Codex backend.
///
/// # Examples
///
/// ```ignore
/// let body = r#"{"models":[{"slug":"gpt-6.1-sol","display_name":"GPT-6.1 Sol",
///   "visibility":"list","context_window":400000}]}"#;
/// let models = parse_models(body.as_bytes()).unwrap();
/// assert_eq!(models[0].slug, "gpt-6.1-sol");
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub(crate) struct DiscoveredModel {
    /// API slug used as the model ID.
    pub slug: String,
    /// Human-readable name.
    #[serde(default)]
    pub display_name: Option<String>,
    /// `list`, `hide`, or `none`; only listed models are offered.
    #[serde(default)]
    pub visibility: Option<String>,
    /// Context window in tokens, when published.
    #[serde(default)]
    pub context_window: Option<i64>,
    /// Reasoning efforts advertised for this account.
    #[serde(default)]
    pub supported_reasoning_levels: Vec<ReasoningEffort>,
    /// Available wire-level service tiers.
    #[serde(default)]
    pub service_tiers: Vec<ServiceTier>,
}

impl DiscoveredModel {
    /// Whether the backend wants this model shown in pickers.
    pub(crate) fn is_listed(&self) -> bool {
        self.visibility.as_deref().is_none_or(|v| v == "list")
    }
}

#[derive(Deserialize)]
struct ModelsResponse {
    models: Vec<DiscoveredModel>,
}

/// Decode a `/models` response body.
///
/// # Errors
///
/// Returns an error when the body is not the expected JSON shape.
pub(crate) fn parse_models(body: &[u8]) -> anyhow::Result<Vec<DiscoveredModel>> {
    Ok(serde_json::from_slice::<ModelsResponse>(body)?.models)
}

#[cfg(test)]
#[path = "model_discovery_wire_tests.rs"]
mod tests;
