//! RLM outcome enum.

use serde::{Deserialize, Serialize};

/// Why an RLM loop finished.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RlmOutcome {
    Converged,
    Exhausted,
    Failed,
    Aborted,
}

impl RlmOutcome {
    /// `true` only for `Converged`.
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Converged)
    }
}
