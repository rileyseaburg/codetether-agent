//! Shared DelegationState contract.
use super::*;

/// Per-session CADMAS-CTX sidecar.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DelegationState {
    /// Posteriors keyed by `"agent|skill|difficulty|dependency|tool_use"`.
    #[serde(default)]
    pub beliefs: BTreeMap<String, BetaPosterior>,
    /// Runtime configuration.
    #[serde(default)]
    pub config: DelegationConfig,
}
