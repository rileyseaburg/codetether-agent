//! Guard that keeps the model from clearing an unverified goal.
//!
//! Clearing a goal would let a worker walk away from an objective without
//! passing the independent verifier. The model may only clear a goal whose
//! status the verifier approved (`complete` or `blocked`); an active,
//! paused, or limit-stopped goal can only be discarded by the user via
//! `/goal clear`.

use crate::session::tasks::{GoalStatus, TaskLog, TaskState};
use crate::tool::ToolResult;
use anyhow::Result;

/// Return a refusal unless the current goal is absent or verifier-approved.
///
/// # Returns
///
/// `Ok(Some(error))` when clearing must be refused, `Ok(None)` otherwise.
///
/// # Errors
///
/// Propagates task-log read failures.
pub(super) async fn refusal(log: &TaskLog) -> Result<Option<ToolResult>> {
    let state = TaskState::from_log(&log.read_all().await?);
    let unverified = state
        .goal
        .is_some_and(|goal| !matches!(goal.status, GoalStatus::Complete | GoalStatus::Blocked));
    Ok(unverified.then(|| {
        ToolResult::error(
            "cannot clear an unverified goal: finish it with update_goal status=complete \
             (or blocked) so the independent verifier can check it; only the user may \
             discard an unfinished or paused goal via /goal clear",
        )
    }))
}
