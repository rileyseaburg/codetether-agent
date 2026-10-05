//! Validation, independent verification, and persistence for `update_goal`.

use super::verify::{VerificationRequest, VerifierAgent};
use super::{context, update::Args};
use crate::session::tasks::GoalStatus;
use crate::tool::ToolResult;
use anyhow::Result;
#[path = "update_commit.rs"]
mod commit;

#[path = "update_start.rs"]
mod start;
pub(super) use start::run;

/// Apply a terminal goal transition only after `verifier` passes it.
pub(super) async fn run_with(args: Args, verifier: &dyn VerifierAgent) -> Result<ToolResult> {
    let status = match args.status.as_str() {
        "complete" => GoalStatus::Complete,
        "blocked" => GoalStatus::Blocked,
        _ => return Ok(ToolResult::error("status must be complete or blocked")),
    };
    let session_id = context::session_id(args.session_id)?;
    let (_, state) = crate::session::tasks::runtime::current(&session_id).await?;
    let Some(goal) = state.goal else {
        return Ok(ToolResult::error("cannot update goal: no goal exists"));
    };
    let request = VerificationRequest::from_goal(&goal, status, &args.evidence);
    if goal.status == GoalStatus::Paused {
        return Ok(ToolResult::error(
            "Goal is paused; resume it explicitly before requesting verification.",
        ));
    }
    if let Some(refused) = super::update_gate::check(&session_id, &goal, &request, verifier).await?
    {
        tracing::info!(session_id = %session_id, claimed = status.as_str(), "Goal transition not authorized");
        return Ok(refused);
    }
    commit::run(&session_id, &goal, status).await
}

#[cfg(test)]
#[path = "update_run_blocked_tests.rs"]
mod blocked_tests;
#[cfg(test)]
#[path = "update_run_cap_tests.rs"]
mod cap_tests;
#[cfg(test)]
#[path = "update_run_tests.rs"]
mod tests;
