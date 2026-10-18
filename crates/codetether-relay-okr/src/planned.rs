//! Deserialization schema for model-authored relay objectives.
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PlannedOkrKeyResult {
    #[serde(default)]
    pub(crate) title: String,
    #[serde(default)]
    pub(crate) target_value: f64,
    #[serde(default = "default_okr_unit")]
    pub(crate) unit: String,
}

#[derive(Debug, Clone, Deserialize)]
/// Model-authored objective with optional JSON fields and quantitative key results.
/// # Examples
/// ```
/// let draft = serde_json::from_str::<codetether_relay_okr::PlannedOkrDraft>("{}");
/// assert!(draft.is_ok());
/// ```
pub struct PlannedOkrDraft {
    #[serde(default)]
    pub(crate) title: String,
    #[serde(default)]
    pub(crate) description: String,
    #[serde(default)]
    pub(crate) key_results: Vec<PlannedOkrKeyResult>,
}

pub(crate) fn default_okr_unit() -> String {
    "%".to_string()
}
