//! API catalog ApiModelInfo representation.
use super::*;

/// Model information from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let model = codetether_model_catalog::ApiModelInfo {
///     id: "example".into(), name: "Example".into(), family: None,
///     attachment: false, reasoning: false, tool_call: true, structured_output: None,
///     temperature: None, knowledge: None, release_date: None, last_updated: None,
///     modalities: None, open_weights: false, cost: None, limit: None,
/// };
/// assert!(model.tool_call);
/// ```
pub struct ApiModelInfo {
    /// Model identifier supplied by the catalog.
    pub id: String,
    /// Display name supplied by the catalog.
    pub name: String,
    #[serde(default)]
    /// Optional model-family label.
    pub family: Option<String>,
    #[serde(default)]
    /// Whether attachment support is advertised.
    pub attachment: bool,
    #[serde(default)]
    /// Whether reasoning support is advertised.
    pub reasoning: bool,
    #[serde(default)]
    /// Whether tool-call support is advertised.
    pub tool_call: bool,
    #[serde(default)]
    /// Optional structured-output capability flag.
    pub structured_output: Option<bool>,
    #[serde(default)]
    /// Optional temperature capability flag.
    pub temperature: Option<bool>,
    #[serde(default)]
    /// Optional knowledge-cutoff label.
    pub knowledge: Option<String>,
    #[serde(default)]
    /// Optional release-date label.
    pub release_date: Option<String>,
    #[serde(default)]
    /// Optional last-update label.
    pub last_updated: Option<String>,
    #[serde(default)]
    /// Optional input/output modality description.
    pub modalities: Option<ModelModalities>,
    #[serde(default)]
    /// Whether open weights are advertised.
    pub open_weights: bool,
    #[serde(default)]
    /// Optional per-token cost data.
    pub cost: Option<ModelCost>,
    #[serde(default)]
    /// Optional token-limit data.
    pub limit: Option<ModelLimit>,
}
