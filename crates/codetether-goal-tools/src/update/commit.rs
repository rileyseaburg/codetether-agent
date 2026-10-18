//! Commit only the goal revision inspected by the independent verifier.
use super::Host;
use anyhow::Result;
use codetether_goal_core::{Goal, GoalStatus};
use codetether_tool_core::ToolResult;

pub(super) async fn run<H: Host>(
    session: &str,
    goal: &Goal,
    status: GoalStatus,
) -> Result<ToolResult> {
    if !H::set_status_if_current(session, goal, status).await? {
        return Ok(ToolResult::error(
            "GOAL_VERIFICATION_STALE: the goal changed or was held during verification. \
             The verifier result was not applied; inspect the current goal before retrying.",
        ));
    }
    let (_, state) = H::current(session).await?;
    Ok(crate::response::result(&state))
}
