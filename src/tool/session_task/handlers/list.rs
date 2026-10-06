//! List handler for the `session_task` tool.

use crate::session::tasks::{TaskLog, TaskState};
use crate::tool::ToolResult;
use anyhow::Result;

pub async fn list(log: &TaskLog) -> Result<ToolResult> {
    let events = log.read_all().await?;
    let state = TaskState::from_log(&events);
    let rendered = crate::session::tasks::task_block(&state);
    Ok(ToolResult::success(rendered))
}
