//! List handler for the `session_task` tool.

use anyhow::Result;
use codetether_goal_core::{TaskLog, TaskState};
use codetether_tool_core::ToolResult;

pub async fn list(log: &TaskLog) -> Result<ToolResult> {
    let events = log.read_all().await?;
    let state = TaskState::from_log(&events);
    let rendered = codetether_goal_core::task_block(&state);
    Ok(ToolResult::success(rendered))
}
