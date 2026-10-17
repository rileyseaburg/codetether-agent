//! update for session todo tools.
use super::*;
pub(super) fn execute(
    tool: &TodoWriteTool,
    params: &Value,
    p: WriteParams,
    mut todos: Vec<TodoItem>,
) -> Result<ToolResult> {
    let id =
        p.id.ok_or_else(|| anyhow::anyhow!("id required for update"))?;
    let todo = todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| anyhow::anyhow!("Todo not found: {}", id))?;

    if let Some(content) = p.content {
        todo.content = content;
    }
    if let Some(status) = p.status {
        todo.status = match status.as_str() {
            "in_progress" | "inprogress" => TodoStatus::InProgress,
            "done" => TodoStatus::Done,
            "blocked" => TodoStatus::Blocked,
            _ => TodoStatus::Pending,
        };
    }
    if let Some(priority) = p.priority {
        todo.priority = match priority.as_str() {
            "low" => Priority::Low,
            "high" => Priority::High,
            "critical" => Priority::Critical,
            _ => Priority::Medium,
        };
    }
    tool.save_todos(&params, &todos)?;
    Ok(ToolResult::success(format!("Updated todo: {}", id)))
}
