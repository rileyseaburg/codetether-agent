//! Source of an execution and its stable string labels.

use serde::{Deserialize, Serialize};

/// Source of an execution: local CLI, worker, Ralph, swarm, or relay.
///
/// # Examples
/// ```
/// use codetether_provenance::{ExecutionOrigin};
/// assert!(matches!(ExecutionOrigin::Worker, ExecutionOrigin::Worker));
/// ```
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionOrigin {
    #[default]
    LocalCli,
    Worker,
    Ralph,
    Swarm,
    Relay,
}

impl ExecutionOrigin {
    ///
    /// # Examples
    /// ```
    /// use codetether_provenance::{ExecutionOrigin};
    /// assert_eq!(ExecutionOrigin::Relay.as_str(),"relay");
    /// ```
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LocalCli => "local_cli",
            Self::Worker => "worker",
            Self::Ralph => "ralph",
            Self::Swarm => "swarm",
            Self::Relay => "relay",
        }
    }
}
