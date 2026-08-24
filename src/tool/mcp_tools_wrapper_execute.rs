//! Approval-claim boundary for one remote MCP tool call.

use super::{McpToolWrapper, ToolResult};
use anyhow::Result;
use serde_json::Value;

pub(super) async fn run(tool: &McpToolWrapper, args: Value) -> Result<ToolResult> {
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation(&tool.id, &args).await {
        return Ok(blocked);
    }
    crate::approval::use_once::claim(&tool.id, &args)
        .map_err(|error| anyhow::anyhow!("approval claim failed: {error}"))?;
    let result = tool
        .client
        .call_tool_authorized(&tool.tool.name, args)
        .await?;
    let output = result
        .content
        .iter()
        .map(render)
        .collect::<Vec<_>>()
        .join("\n");
    if result.is_error {
        Ok(ToolResult::error(output))
    } else {
        Ok(ToolResult::success(output))
    }
}

fn render(item: &crate::mcp::ToolContent) -> String {
    match item {
        crate::mcp::ToolContent::Text { text } => text.clone(),
        crate::mcp::ToolContent::Image { data, mime_type } => {
            format!("[image: {mime_type} ({} bytes)]", data.len())
        }
        crate::mcp::ToolContent::Resource { resource } => {
            serde_json::to_string(resource).unwrap_or_default()
        }
    }
}
