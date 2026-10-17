//! RalphStatus serializable contract.
use super::*;

/// Ralph execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
///
/// # Examples
/// ```
/// use codetether_ralph_core::RalphStatus;
/// let status = RalphStatus::Pending;
/// match status { RalphStatus::Pending => {}, RalphStatus::Running => {}, RalphStatus::Completed => {}, RalphStatus::MaxIterations => {}, RalphStatus::Stopped => {}, RalphStatus::QualityFailed => {} }
/// ```
pub enum RalphStatus {
    /// Awaiting execution.
    Pending,
    /// Currently executing stories.
    Running,
    /// All planned work completed.
    Completed,
    /// Stopped at the iteration limit.
    MaxIterations,
    /// Execution stopped before completion.
    Stopped,
    /// Final quality checks failed.
    QualityFailed,
}
