//! list for in-memory task tracking.
use super::*;
pub(super) fn run(_p: Params) -> Result<ToolResult> {
    let store = TASK_STORE.read();
    if store.is_empty() {
        return Ok(ToolResult::success("No active tasks".to_string()));
    }

    let output = store
        .values()
        .map(|t| {
            let icon = match t.status {
                ToolTaskStatus::Pending => "○",
                ToolTaskStatus::Running => "◐",
                ToolTaskStatus::Complete => "●",
                ToolTaskStatus::Failed => "✗",
            };
            format!("{} {} - {}", icon, t.id, t.description)
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(ToolResult::success(output).with_metadata("count", json!(store.len())))
}
