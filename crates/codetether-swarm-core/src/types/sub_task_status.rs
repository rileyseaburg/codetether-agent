//! SubTaskStatus shared contract.
use super::*;

/// Status of a subtask
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
///
/// # Examples
/// ```
/// use codetether_swarm_core::SubTaskStatus;
/// let status = SubTaskStatus::Pending;
/// match status { SubTaskStatus::Pending => {}, SubTaskStatus::Blocked => {}, SubTaskStatus::Running => {}, SubTaskStatus::Completed => {}, SubTaskStatus::Failed => {}, SubTaskStatus::Cancelled => {}, SubTaskStatus::TimedOut => {} }
/// ```
pub enum SubTaskStatus {
    /// Waiting to be scheduled
    Pending,

    /// Waiting for dependencies
    Blocked,

    /// Currently executing
    Running,

    /// Successfully completed
    Completed,

    /// Failed with error
    Failed,

    /// Cancelled by orchestrator
    Cancelled,

    /// Timed out
    TimedOut,
}
