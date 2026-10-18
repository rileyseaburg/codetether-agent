//! Handle the MCP tools subcommand.
use crate::CommandOutput;
use codetether_mcp_connections::{ConnectionClient, Connections};
pub(crate) async fn run<C: ConnectionClient>(
    registry: &Connections<C>,
    value: &str,
    response: &mut CommandOutput,
) {
    let server = Some(value.trim()).filter(|s| !s.is_empty());
    match registry.list_tools(server).await {
        Ok(tools) => {
            if tools.is_empty() {
                response.status = "No MCP tools available".to_string();
                response.message = Some(("No MCP tools available.").into());
            } else {
                response.status = format!("{} MCP tool(s) available", tools.len());
                let body = tools
                    .into_iter()
                    .map(|(server_name, tool)| {
                        let description = tool
                            .description
                            .unwrap_or_else(|| "Remote MCP tool".to_string());
                        format!("- [{server_name}] {} — {}", tool.name, description)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                response.message = Some((format!("Available MCP tools:\n{body}")).into());
            }
        }
        Err(error) => {
            response.status = format!("MCP tools failed: {error}");
            response.message = Some((format!("Failed to list MCP tools: {error}")).into());
        }
    }
}
