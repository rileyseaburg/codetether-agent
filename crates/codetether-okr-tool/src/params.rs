//! params for OKR tool dispatch.
use super::*;

#[derive(Deserialize)]
pub(crate) struct Params {
    pub(crate) action: String,
    #[serde(default)]
    pub(crate) id: Option<String>,
    #[serde(default)]
    pub(crate) title: Option<String>,
    #[serde(default)]
    pub(crate) description: Option<String>,
    #[serde(default)]
    pub(crate) status: Option<String>,
    #[serde(default)]
    pub(crate) owner: Option<String>,
    #[serde(default)]
    pub(crate) tenant_id: Option<String>,
    #[serde(default)]
    pub(crate) key_results: Option<Vec<KrParam>>,
    // Run-specific fields
    #[serde(default)]
    pub(crate) okr_id: Option<String>,
    #[serde(default)]
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) correlation_id: Option<String>,
    #[serde(default)]
    pub(crate) session_id: Option<String>,
    #[serde(default)]
    pub(crate) checkpoint_id: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct KrParam {
    pub(crate) title: String,
    pub(crate) target_value: f64,
    #[serde(default = "default_unit")]
    pub(crate) unit: String,
}

fn default_unit() -> String {
    "%".to_string()
}
