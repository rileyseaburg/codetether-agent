//! Parse bridge commands, authorize them and dispatch a single action.
use crate::Host;
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) async fn run<H: Host>(args: Value) -> Result<ToolResult> {
    let action = args["action"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing 'action' parameter"))?;
    let command = args["command"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing 'command' parameter"))?;

    // Parse command into parts
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(ToolResult::error("Empty command"));
    }

    let (cmd, cmd_args) = (parts[0], parts[1..].to_vec());
    let approval_id = args["approval_id"].as_str();
    if let Some(blocked) = super::policy::blocked::<H>(cmd, &cmd_args, approval_id).await {
        return Ok(blocked);
    }

    match action {
        "list_tools" => super::list_tools::run::<H>(cmd, &cmd_args, approval_id).await,
        "call_tool" => super::call_tool::run::<H>(cmd, &cmd_args, approval_id, &args).await,
        "list_resources" => super::list_resources::run::<H>(cmd, &cmd_args, approval_id).await,
        "read_resource" => super::read_resource::run::<H>(cmd, &cmd_args, approval_id, &args).await,
        _ => Ok(ToolResult::error(format!("Unknown action: {}", action))),
    }
}
