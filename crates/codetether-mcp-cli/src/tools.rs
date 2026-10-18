//! MCP tool metadata rendering without changing remote or native schemas.
use codetether_mcp_core::types::McpTool;
#[path = "tools_view.rs"]
mod view;
use view::View;
pub(super) fn connected(tools: &[McpTool], json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(&tools)?);
    } else {
        println!("# Connected to MCP Server\n");
        println!("## Available Tools ({})\n", tools.len());
        for tool in tools {
            println!(
                "- **{}**: {}",
                tool.name,
                tool.description.as_deref().unwrap_or("")
            );
        }
    }

    Ok(())
}
pub(super) fn available<T: serde::Serialize + View>(tools: &[T], json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(&tools)?);
    } else {
        println!("# Available MCP Tools\n");
        for tool in tools {
            println!(
                "- **{}**: {}",
                tool.name(),
                tool.description().unwrap_or("")
            );
        }
    }
    Ok(())
}
