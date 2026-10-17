//! add for session todo tools.
use super::*;
pub(super) fn execute(
    tool: &TodoWriteTool,
    params: &Value,
    p: WriteParams,
    mut todos: Vec<TodoItem>,
) -> Result<ToolResult> {
    let content = p
        .content
        .ok_or_else(|| anyhow::anyhow!("content required for add"))?;
    let status = p
        .status
        .map(|s| match s.as_str() {
            "in_progress" | "inprogress" => TodoStatus::InProgress,
            "done" => TodoStatus::Done,
            "blocked" => TodoStatus::Blocked,
            _ => TodoStatus::Pending,
        })
        .unwrap_or_default();
    let priority = p
        .priority
        .map(|s| match s.as_str() {
            "low" => Priority::Low,
            "high" => Priority::High,
            "critical" => Priority::Critical,
            _ => Priority::Medium,
        })
        .unwrap_or_default();

    let id = tool.generate_id();
    todos.push(TodoItem {
        id: id.clone(),
        content,
        status,
        priority,
        created_at: Some(chrono::Utc::now().to_rfc3339()),
    });
    tool.save_todos(&params, &todos)?;
    Ok(ToolResult::success(format!("Added todo: {}", id)))
}
