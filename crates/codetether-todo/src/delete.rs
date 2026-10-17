//! delete for session todo tools.
use super::*;
pub(super) fn execute(
    tool: &TodoWriteTool,
    params: &Value,
    p: WriteParams,
    mut todos: Vec<TodoItem>,
) -> Result<ToolResult> {
    let id =
        p.id.ok_or_else(|| anyhow::anyhow!("id required for delete"))?;
    let len_before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() == len_before {
        return Ok(ToolResult::error(format!("Todo not found: {}", id)));
    }
    tool.save_todos(&params, &todos)?;
    Ok(ToolResult::success(format!("Deleted todo: {}", id)))
}
