//! Validate the requested action, file and position before dispatch.
use super::args::{get_file_path_arg, resolve_action_raw};
use super::{Host, LspOperation, LspTool, capability_gate};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::path::Path;
pub(super) async fn execute<H: Host>(tool: &LspTool<H>, args: Value) -> Result<ToolResult> {
    let action_raw = resolve_action_raw(&args)?;
    let action = LspOperation::parse(&action_raw)
        .ok_or_else(|| anyhow::anyhow!("Unknown action: {}", action_raw))?;

    let manager = tool.get_manager().await;

    if action == LspOperation::WorkspaceSymbol {
        return super::workspace::execute(&manager, &args).await;
    }
    let file_path = get_file_path_arg(&args).ok_or_else(|| {
        anyhow::anyhow!(
            "file_path is required (or use path) for action: {}",
            action_raw
        )
    })?;
    let path = Path::new(file_path);

    let client = manager.get_client_for_file(path).await?;
    if let Err(reason) = capability_gate::check(action, client.capabilities().await.as_ref()) {
        return Ok(ToolResult::error(reason));
    }

    let line = args["line"].as_u64().map(|l| l as u32);
    let column = args["column"].as_u64().map(|c| c as u32);

    if action.requires_position() && (line.is_none() || column.is_none()) {
        return Ok(ToolResult::error(format!(
            "line and column are required for action: {}",
            action.canonical_name()
        )));
    }

    super::dispatch::run(&*client, &manager, path, action, &args, line, column).await
}
