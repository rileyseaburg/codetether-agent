//! Dispatch catalog reads and calls after session initialization.
use super::{catalog::catalog, input::Input};
use crate::client::Client;
use anyhow::{Result, ensure};
use codetether_tool_core::ToolResult;
use serde_json::json;

pub(super) async fn run(client: &mut Client, input: Input) -> Result<ToolResult> {
    client.initialize().await?;
    let tools = catalog(client).await?;
    match input {
        Input::ListTools {} => Ok(ToolResult::success(
            json!({"tools":tools,"mode":"read_only"}).to_string(),
        )),
        Input::Call { tool, arguments } => {
            ensure!(
                arguments.is_object(),
                "Monarch tool arguments must be an object"
            );
            ensure!(
                tools.iter().any(|t| t["name"].as_str() == Some(&tool)),
                "Tool not found or not explicitly read-only; writes are disabled"
            );
            let result = client
                .rpc("tools/call", json!({"name":tool,"arguments":arguments}))
                .await?;
            let failed = result["isError"].as_bool().unwrap_or(false);
            let output = serde_json::to_string(&result)?;
            Ok(if failed {
                ToolResult::error(output)
            } else {
                ToolResult::success(output)
            })
        }
        Input::Status {} => unreachable!("status handled before network access"),
    }
}
