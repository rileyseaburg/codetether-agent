//! SubAgentStatus execution contract.
use super::*;

/// Status of a sub-agent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
///
/// # Examples
/// ```
/// use codetether_swarm_core::SubAgentStatus;
/// let status = SubAgentStatus::Initializing;
/// match status { SubAgentStatus::Initializing => {}, SubAgentStatus::Working => {}, SubAgentStatus::Waiting => {}, SubAgentStatus::Completed => {}, SubAgentStatus::Failed => {}, SubAgentStatus::Terminated => {} }
/// ```
pub enum SubAgentStatus {
    /// Being initialized
    Initializing,

    /// Actively working
    Working,

    /// Waiting for resource/dependency
    Waiting,

    /// Successfully completed
    Completed,

    /// Failed
    Failed,

    /// Terminated by orchestrator
    Terminated,
}
