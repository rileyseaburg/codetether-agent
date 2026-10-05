//! Escalation when a goal exhausts its verifier attempts.

use super::verdict_log::{self, VerdictRecord};
use crate::session::tasks::{Goal, GoalStatus};
use crate::tool::ToolResult;
use anyhow::Result;

/// Record the cap marker, pause the goal, and tell the worker to stop.
///
/// The marker resets the rejection count, so `/goal resume` grants a fresh
/// set of attempts after the user reviews the goal.
///
/// # Errors
///
/// Propagates verdict-log and task-log write failures.
pub(super) async fn escalate(
    session: &str,
    goal: &Goal,
    claimed: &str,
    cap: usize,
) -> Result<ToolResult> {
    if !crate::session::tasks::runtime::set_status_if_current(session, goal, GoalStatus::Paused)
        .await?
    {
        return Ok(ToolResult::error(
            "GOAL_VERIFICATION_STALE: goal changed before escalation; no newer goal was paused.",
        ));
    }
    let mut marker = VerdictRecord::new(&goal.id, claimed, false, "attempt-cap", "");
    marker.escalated = true;
    verdict_log::append(session, &marker).await?;
    tracing::warn!(session_id = %session, cap, "Goal verification attempt cap reached; paused");
    Ok(ToolResult::error(format!(
        "The verifier rejected this goal {cap} times in a row. The goal is now paused and \
         needs user review; stop and report the outstanding findings to the user."
    )))
}
