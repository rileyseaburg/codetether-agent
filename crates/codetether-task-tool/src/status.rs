//! status for in-memory task tracking.
use super::*;
pub(super) fn run(p: Params) -> Result<ToolResult> {
    let id = p.id.ok_or_else(|| anyhow::anyhow!("id required"))?;
    let store = TASK_STORE.read();

    match store.get(&id) {
        Some(task) => {
            let status_str = match task.status {
                ToolTaskStatus::Pending => "pending",
                ToolTaskStatus::Running => "running",
                ToolTaskStatus::Complete => "complete",
                ToolTaskStatus::Failed => "failed",
            };
            Ok(ToolResult::success(format!(
                "Task {} [{}]: {}\n{}",
                task.id,
                status_str,
                task.description,
                task.result.as_deref().unwrap_or("")
            )))
        }
        None => Ok(ToolResult::error(format!("Task not found: {}", id))),
    }
}
