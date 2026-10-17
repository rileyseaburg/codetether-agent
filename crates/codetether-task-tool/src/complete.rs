//! complete for in-memory task tracking.
use super::*;
pub(super) fn run(p: Params) -> Result<ToolResult> {
    let id = p.id.ok_or_else(|| anyhow::anyhow!("id required"))?;
    let result = p.result.unwrap_or_else(|| "Completed".to_string());

    let mut store = TASK_STORE.write();
    match store.get_mut(&id) {
        Some(task) => {
            task.status = ToolTaskStatus::Complete;
            task.result = Some(result.clone());
            Ok(ToolResult::success(format!("Completed task: {}", id)))
        }
        None => Ok(ToolResult::error(format!("Task not found: {}", id))),
    }
}
