//! create for in-memory task tracking.
use super::*;
pub(super) fn run(p: Params) -> Result<ToolResult> {
    let description = p
        .description
        .ok_or_else(|| anyhow::anyhow!("description required"))?;
    let id = format!("task_{}", TASK_COUNTER.fetch_add(1, Ordering::SeqCst));

    let task = TaskInfo {
        id: id.clone(),
        description: description.clone(),
        status: ToolTaskStatus::Pending,
        result: None,
    };

    TASK_STORE.write().insert(id.clone(), task);
    Ok(
        ToolResult::success(format!("Created task: {} - {}", id, description))
            .with_metadata("task_id", json!(id)),
    )
}
