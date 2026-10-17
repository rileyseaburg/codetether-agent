//! execute for in-memory task tracking.
use super::*;
pub(super) async fn execute(params: Value) -> Result<ToolResult> {
    let p: Params = serde_json::from_value(params).context("Invalid params")?;
    match p.action.as_str() {
        "create" => create::run(p),
        "status" => status::run(p),
        "complete" => complete::run(p),
        "cancel" => cancel::run(p),
        "list" => list::run(p),
        _ => Ok(ToolResult::error(format!("Unknown action: {}", p.action))),
    }
}
