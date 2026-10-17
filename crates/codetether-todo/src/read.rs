//! read for session todo tools.
use super::*;
pub(super) fn execute(tool: &TodoReadTool, params: Value) -> Result<ToolResult> {
    let p: ReadParams = serde_json::from_value(params.clone()).unwrap_or(ReadParams {
        status: None,
        priority: None,
    });

    let todos = tool.load_todos(&params)?;

    let filtered: Vec<&TodoItem> = todos.iter().filter(|t| filter::matches(t, &p)).collect();

    if filtered.is_empty() {
        return Ok(ToolResult::success("No todos found".to_string()));
    }

    let output = render::format(&filtered);

    Ok(ToolResult::success(output).with_metadata("count", json!(filtered.len())))
}
