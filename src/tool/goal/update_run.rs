//! Validation and persistence for `update_goal`.

use super::{context, response, update::Args};
use crate::session::tasks::GoalStatus;
use crate::tool::ToolResult;
use anyhow::Result;

pub(super) async fn run(args: Args) -> Result<ToolResult> {
    let status = match args.status.as_str() {
        "complete" => GoalStatus::Complete,
        "blocked" => GoalStatus::Blocked,
        _ => return Ok(ToolResult::error("status must be complete or blocked")),
    };
    let session_id = context::session_id(args.session_id)?;
    let (_, state) = crate::session::tasks::runtime::current(&session_id).await?;
    if state.goal.is_none() {
        return Ok(ToolResult::error("cannot update goal: no goal exists"));
    }
    crate::session::tasks::runtime::set_status(&session_id, status).await?;
    let (_, state) = crate::session::tasks::runtime::current(&session_id).await?;
    Ok(response::result(&state))
}
