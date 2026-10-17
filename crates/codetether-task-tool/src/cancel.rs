//! cancel for in-memory task tracking.
use super::*;
pub(super) fn run(p: Params) -> Result<ToolResult> {
    let id = p.id.ok_or_else(|| anyhow::anyhow!("id required"))?;

    let mut store = TASK_STORE.write();
    match store.get_mut(&id) {
        Some(task) => {
            task.status = ToolTaskStatus::Failed;
            task.result = Some("Cancelled".to_string());
            Ok(ToolResult::success(format!("Cancelled task: {}", id)))
        }
        None => Ok(ToolResult::error(format!("Task not found: {}", id))),
    }
}
