//! Per-action handlers for the `session_task` tool.

use super::params::Params;
use crate::session::tasks::{TaskEvent, TaskLog, TaskState, TaskStatus};
use crate::tool::ToolResult;
use anyhow::{Result, anyhow};
use chrono::Utc;

fn parse_status(s: &str) -> Result<TaskStatus> {
    Ok(match s {
        "pending" => TaskStatus::Pending,
        "in_progress" | "inprogress" => TaskStatus::InProgress,
        "done" => TaskStatus::Done,
        "blocked" => TaskStatus::Blocked,
        "cancelled" | "canceled" => TaskStatus::Cancelled,
        other => return Err(anyhow!("unknown status: {other}")),
    })
}

pub(super) async fn set_goal(log: &TaskLog, p: Params) -> Result<ToolResult> {
    let objective = p
        .objective
        .ok_or_else(|| anyhow!("`objective` is required"))?;
    log.append(&TaskEvent::GoalSet {
        at: Utc::now(),
        objective: objective.clone(),
        success_criteria: p.success_criteria.unwrap_or_default(),
        forbidden: p.forbidden.unwrap_or_default(),
    })
    .await?;
    Ok(ToolResult::success(format!("Goal set: {objective}")))
}

pub(super) async fn reaffirm(log: &TaskLog, p: Params) -> Result<ToolResult> {
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

pub(super) async fn clear_goal(log: &TaskLog, p: Params) -> Result<ToolResult> {
    let reason = p.reason.unwrap_or_else(|| "completed".to_string());
    log.append(&TaskEvent::GoalCleared {
        at: Utc::now(),
        reason: reason.clone(),
    })
    .await?;
    Ok(ToolResult::success(format!("Goal cleared: {reason}")))
}

pub(super) async fn task_add(log: &TaskLog, p: Params) -> Result<ToolResult> {
    let content = p.content.ok_or_else(|| anyhow!("`content` is required"))?;
    let id =
        p.id.unwrap_or_else(|| format!("t{}", Utc::now().timestamp_millis()));
    log.append(&TaskEvent::TaskAdded {
        at: Utc::now(),
        id: id.clone(),
        content: content.clone(),
        parent_id: p.parent_id,
    })
    .await?;
    Ok(ToolResult::success(format!("Added task {id}: {content}")))
}

pub(super) async fn task_status(log: &TaskLog, p: Params) -> Result<ToolResult> {
    let id = p.id.ok_or_else(|| anyhow!("`id` is required"))?;
    let status_str = p.status.ok_or_else(|| anyhow!("`status` is required"))?;
    let status = parse_status(&status_str)?;
    log.append(&TaskEvent::TaskStatus {
        at: Utc::now(),
        id: id.clone(),
        status,
        note: p.note,
    })
    .await?;
    Ok(ToolResult::success(format!("Task {id} → {status_str}")))
}

pub(super) async fn list(log: &TaskLog) -> Result<ToolResult> {
    let events = log.read_all().await?;
    let state = TaskState::from_log(&events);
    let rendered = crate::session::tasks::governance_block(&state)
        .unwrap_or_else(|| "No goal and no tasks.".to_string());
    Ok(ToolResult::success(rendered))
}
