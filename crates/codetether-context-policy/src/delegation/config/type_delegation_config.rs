//! Shared DelegationConfig contract.
use super::*;

/// Tunable knobs for [`DelegationState`](crate::delegation::DelegationState).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationConfig {
    #[serde(default = "default_gamma")]
    pub gamma: f64,
    #[serde(default = "default_delta")]
    pub delta: f64,
    #[serde(default = "default_kappa")]
    pub kappa: f64,
    #[serde(default = "default_lambda")]
    pub lambda: f64,
    #[serde(default)]
    pub enabled: bool,
}
