//! ModelPreferences wire contract and construction.
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelPreferences {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hints: Vec<ModelHint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_priority: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed_priority: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intelligence_priority: Option<f32>,
}
