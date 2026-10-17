//! execute for shared swarm results.
use super::*;
pub(super) async fn execute(tool: &SwarmShareTool, params: Value) -> Result<ToolResult> {
    let p: Params = serde_json::from_value(params).context("Invalid params")?;
    match p.action.as_str() {
        "publish" => tool.publish(p).await,
        "get" => tool.get(p).await,
        "query_tags" => tool.query_tags(p).await,
        "query_prefix" => tool.query_prefix(p).await,
        "list" => tool.list(p).await,
        _ => Ok(ToolResult::error(format!("Unknown action: {}", p.action))),
    }
}
