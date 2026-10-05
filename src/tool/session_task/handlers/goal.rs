//! Goal event handlers for the `session_task` tool.

use super::super::params::Params;
use crate::session::tasks::{GoalReaffirmation, TaskEvent, TaskLog};
use crate::tool::ToolResult;
use anyhow::{Result, anyhow};
use chrono::Utc;

pub async fn reaffirm(log: &TaskLog, p: Params) -> Result<ToolResult> {
    let note = p
        .progress_note
        .ok_or_else(|| anyhow!("`progress_note` is required"))?;
    log.append(&TaskEvent::GoalReaffirmed(GoalReaffirmation {
        at: Utc::now(),
        progress_note: note.clone(),
    }))
    .await?;
    Ok(ToolResult::success(format!("Reaffirmed: {note}")))
}

pub async fn clear_goal(log: &TaskLog, p: Params) -> Result<ToolResult> {
    if let Some(refused) = super::clear_guard::refusal(log).await? {
        return Ok(refused);
    }
    let reason = p.reason.unwrap_or_else(|| "completed".to_string());
    log.append(&TaskEvent::GoalCleared {
        at: Utc::now(),
        reason: reason.clone(),
    })
    .await?;
    Ok(ToolResult::success(format!("Goal cleared: {reason}")))
}
