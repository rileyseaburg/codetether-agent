//! Session task lifecycle status.

use serde::{Deserialize, Serialize};

/// Lifecycle status for a session-governance task.
///
/// `SessionTaskStatus` is persisted inside task-log status transition events
/// and later folded into the materialized session task list. The status controls
/// whether a task is still considered actionable, how it is rendered in
/// governance prompts, and whether later status events should be interpreted as
/// progress, completion, cancellation, or a blocker.
///
/// Values serialize as `snake_case` so the append-only JSONL task log remains
/// stable and readable across releases.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionTaskStatus {
    /// The task has been recorded but work has not started.
    Pending,
    /// The task is actively being worked on.
    InProgress,
    /// The task completed successfully.
    Done,
    /// The task cannot currently proceed because of missing information,
    /// unavailable dependencies, or another explicit blocker.
    Blocked,
    /// The task was intentionally abandoned, superseded, or closed without
    /// successful completion.
    Cancelled,
}
impl SessionTaskStatus {
    /// Returns whether this status represents actionable work.
    ///
    /// Open tasks are the tasks that should still appear as active work in
    /// session-governance rendering. A task is open while it is waiting to start
    /// or actively in progress. Completed, blocked, and cancelled tasks are not
    /// considered open by this helper.
    ///
    /// # Returns
    ///
    /// `true` for [`SessionTaskStatus::Pending`] and
    /// [`SessionTaskStatus::InProgress`]; `false` for
    /// [`SessionTaskStatus::Done`], [`SessionTaskStatus::Blocked`], and
    /// [`SessionTaskStatus::Cancelled`].
    pub fn is_open(&self) -> bool {
        matches!(self, Self::Pending | Self::InProgress)
    }
}
