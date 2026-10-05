//! Commit only the goal revision inspected by the independent verifier.
use crate::session::tasks::{Goal, GoalStatus, runtime};
use crate::tool::ToolResult;
use anyhow::Result;

pub(super) async fn run(session: &str, goal: &Goal, status: GoalStatus) -> Result<ToolResult> {
    if !runtime::set_status_if_current(session, goal, status).await? {
        return Ok(ToolResult::error(
            "GOAL_VERIFICATION_STALE: the goal changed or was held during verification. \
             The verifier result was not applied; inspect the current goal before retrying.",
        ));
    }
    let (_, state) = runtime::current(session).await?;
    Ok(super::super::response::result(&state))
}
