//! write for session todo tools.
use super::*;
pub(super) fn execute(tool: &TodoWriteTool, params: Value) -> Result<ToolResult> {
    let p: WriteParams = serde_json::from_value(params.clone()).context("Invalid params")?;
    let todos = tool.load_todos(&params)?;
    match p.action.as_str() {
        "add" => add::execute(tool, &params, p, todos),
        "update" => update::execute(tool, &params, p, todos),
        "delete" => delete::execute(tool, &params, p, todos),
        "clear" => clear::execute(tool, &params, p, todos),
        _ => Ok(ToolResult::error(format!("Unknown action: {}", p.action))),
    }
}
