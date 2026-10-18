//! Planned objective and key-result schemas.
use serde::Deserialize;

/// Quantitative key result supplied by the planner.
/// # Examples
/// ```
/// let result: codetether_relay_plan::PlannedOkrKeyResult = serde_json::from_str("{}").unwrap();
/// assert_eq!(result.unit, "%");
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct PlannedOkrKeyResult {
    #[serde(default)]
    /// Title supplied by the planner.
    pub title: String,
    #[serde(default)]
    /// Numeric target, clamped during objective construction.
    pub target_value: f64,
    #[serde(default = "default_okr_unit")]
    /// Measurement unit, defaulting to percent when absent.
    pub unit: String,
}

/// Planned objective and its candidate measurable key results.
/// # Examples
/// ```
/// let draft: codetether_relay_plan::PlannedOkrDraft = serde_json::from_str("{}").unwrap();
/// assert!(draft.key_results.is_empty());
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct PlannedOkrDraft {
    #[serde(default)]
    /// Title supplied by the planner.
    pub title: String,
    #[serde(default)]
    /// Objective description supplied by the planner.
    pub description: String,
    #[serde(default)]
    /// Candidate key results in planner order.
    pub key_results: Vec<PlannedOkrKeyResult>,
}

pub(crate) fn default_okr_unit() -> String {
    "%".to_string()
}
