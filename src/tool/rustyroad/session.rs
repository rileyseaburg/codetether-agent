//! MCP initialization, discovery and dispatch over the isolated process.

use super::{
    initialize,
    params::{Action, Params},
    rpc::Rpc,
};
use crate::{
    mcp::{CallToolResult, ListToolsResult},
    tool::ToolResult,
};
use anyhow::{Result, bail};
use serde_json::json;

pub(super) async fn execute(rpc: &mut Rpc, params: &Params) -> Result<ToolResult> {
    let server = initialize::run(rpc).await?;
    let tools: ListToolsResult =
        serde_json::from_value(rpc.request("tools/list", json!({})).await?)?;
    let result = match params.action {
        Action::ListTools => ToolResult::success(serde_json::to_string_pretty(&tools.tools)?),
        Action::CallTool => {
            let name = params.tool_name.as_deref().unwrap_or_default();
            if !tools.tools.iter().any(|tool| tool.name == name) {
                bail!(
                    "RustyRoad does not advertise {name}; use list_tools for this installed version"
                );
            }
            let result: CallToolResult = serde_json::from_value(
                rpc.request(
                    "tools/call",
                    json!({"name": name, "arguments": params.arguments}),
                )
                .await?,
            )?;
            crate::tool::mcp_tools::convert::result(result)
        }
    };
    Ok(result.with_metadata("rustyroad_version", json!(server.version)))
}
