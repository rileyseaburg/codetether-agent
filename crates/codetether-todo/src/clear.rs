//! Clear all todos in the selected session.
use super::*;
pub(super) fn execute(
    tool: &TodoWriteTool,
    params: &Value,
    _p: WriteParams,
    mut todos: Vec<TodoItem>,
) -> Result<ToolResult> {
    let count = todos.len();
    todos.clear();
    tool.save_todos(&params, &todos)?;
    Ok(ToolResult::success(format!("Cleared {} todos", count)))
}
