//! Action routing for the legacy session task surface.

use super::handlers::{clear_goal, complete_goal, list, reaffirm, set_goal, task_add, task_status};
use super::params::Params;
use crate::session::tasks::TaskLog;
use crate::tool::ToolResult;
use anyhow::Result;

pub(super) async fn run(log: &TaskLog, params: Params) -> Result<ToolResult> {
    match canonical(&params.action) {
        "set_goal" => set_goal(log, params).await,
        "reaffirm" => reaffirm(log, params).await,
        "clear_goal" => clear_goal(log, params).await,
        "update_goal_complete" => complete_goal(params).await,
        "task_add" => task_add(log, params).await,
        "task_status" => task_status(log, params).await,
        "list" => list(log).await,
        other => Ok(ToolResult::error(format!("unknown action: {other}"))),
    }
}

fn canonical(action: &str) -> &str {
    match action {
        "complete_goal" => "update_goal_complete",
        other => other,
    }
}

#[cfg(test)]
#[path = "dispatch_tests.rs"]
mod tests;
