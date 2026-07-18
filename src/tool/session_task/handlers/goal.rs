//! Goal event handlers for the `session_task` tool.

use super::super::params::Params;
use crate::session::tasks::{TaskEvent, TaskLog};
use crate::tool::ToolResult;
use anyhow::{Result, anyhow};
use chrono::Utc;

pub async fn set_goal(log: &TaskLog, p: Params) -> Result<ToolResult> {
    let objective = p
        .objective
        .ok_or_else(|| anyhow!("`objective` is required"))?;
    log.append(&TaskEvent::GoalSet {
        at: Utc::now(),
        goal_id: uuid::Uuid::new_v4().to_string(),
        objective: objective.clone(),
        success_criteria: p.success_criteria.unwrap_or_default(),
        forbidden: p.forbidden.unwrap_or_default(),
        source_session_id: String::new(),
        source_turn_id: String::new(),
        source_text_hash: String::new(),
        source_kind: Default::default(),
        confidence: 0.0,
    })
    .await?;
    Ok(ToolResult::success(format!("Goal set: {objective}")))
}

pub async fn reaffirm(log: &TaskLog, p: Params) -> Result<ToolResult> {
    let note = p
        .progress_note
        .ok_or_else(|| anyhow!("`progress_note` is required"))?;
    log.append(&TaskEvent::GoalReaffirmed {
        at: Utc::now(),
        progress_note: note.clone(),
    })
    .await?;
    Ok(ToolResult::success(format!("Reaffirmed: {note}")))
}

pub async fn clear_goal(log: &TaskLog, p: Params) -> Result<ToolResult> {
    let reason = p.reason.unwrap_or_else(|| "completed".to_string());
    log.append(&TaskEvent::GoalCleared {
        at: Utc::now(),
        reason: reason.clone(),
    })
    .await?;
    Ok(ToolResult::success(format!("Goal cleared: {reason}")))
}
