//! Delegation config tunables and defaults.

use serde::{Deserialize, Serialize};

/// Default uncertainty penalty `γ` for LCB scoring (CADMAS-CTX §3.4).
pub const DEFAULT_GAMMA: f64 = 0.5;
/// Default delegation margin `δ`.
pub const DEFAULT_DELTA: f64 = 0.05;
/// Default weak-prior strength `κ`.
pub const DEFAULT_KAPPA: f64 = 2.0;
/// Default forgetting factor `λ` (1.0 = disabled).
pub const DEFAULT_LAMBDA: f64 = 1.0;

fn default_gamma() -> f64 {
    DEFAULT_GAMMA
}
fn default_delta() -> f64 {
    DEFAULT_DELTA
}
fn default_kappa() -> f64 {
    DEFAULT_KAPPA
}
fn default_lambda() -> f64 {
    DEFAULT_LAMBDA
}

/// Tunable knobs for [`DelegationState`](super::DelegationState).
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

impl Default for DelegationConfig {
    fn default() -> Self {
        Self {
            gamma: DEFAULT_GAMMA,
            delta: DEFAULT_DELTA,
            kappa: DEFAULT_KAPPA,
            lambda: DEFAULT_LAMBDA,
            enabled: false,
        }
    }
}
