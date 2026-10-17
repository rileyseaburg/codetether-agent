//! OkrRunStatus serialized state.

use serde::{Deserialize, Serialize};

/// Status of an OKR run
///
/// # Examples
/// ```
/// use codetether_okr::{OkrRunStatus};
/// let value = OkrRunStatus::Draft;
/// assert!(matches!(value, OkrRunStatus::Draft));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum OkrRunStatus {
    /// Draft - not yet submitted
    #[default]
    Draft,

    /// Pending approval
    PendingApproval,

    /// Approved to run
    Approved,

    /// Actively running
    Running,

    /// Paused (can resume)
    Paused,

    /// Waiting for something
    WaitingApproval,

    /// Successfully completed
    Completed,

    /// Failed to complete
    Failed,

    /// Denied approval
    Denied,

    /// Cancelled
    Cancelled,
}
