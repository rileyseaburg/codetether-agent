//! Task event handlers for the `session_task` tool.

use super::super::params::Params;
use super::status_parse::parse_status;
use crate::session::tasks::{TaskEvent, TaskLog};
use crate::tool::ToolResult;
use anyhow::{Result, anyhow};
use chrono::Utc;

pub async fn task_add(log: &TaskLog, p: Params) -> Result<ToolResult> {
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

pub async fn task_status(log: &TaskLog, p: Params) -> Result<ToolResult> {
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
