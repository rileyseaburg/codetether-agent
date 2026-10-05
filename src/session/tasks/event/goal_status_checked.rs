//! Compare-and-set lifecycle events for decisions made asynchronously.
use super::GoalStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Apply a status only to the exact goal revision that was inspected.
///
/// Replay ignores stale identities, revisions, and active answer-review holds.
///
/// # Examples
/// ```
/// use chrono::Utc;
/// use codetether_agent::session::tasks::{GoalStatusChecked, GoalStatus};
/// let update = GoalStatusChecked {
///     at: Utc::now(), goal_id: "g".into(), expected_updated_at: Utc::now(),
///     status: GoalStatus::Complete,
/// };
/// assert_eq!(update.status, GoalStatus::Complete);
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoalStatusChecked {
    /// Time this decision was appended.
    pub at: DateTime<Utc>,
    /// Original goal identity, never re-resolved after verification.
    pub goal_id: String,
    /// Revision captured before starting the asynchronous operation.
    pub expected_updated_at: DateTime<Utc>,
    /// Requested lifecycle state.
    pub status: GoalStatus,
}
