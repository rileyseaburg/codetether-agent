//! Handle the MCP servers subcommand.
use crate::CommandOutput;
use codetether_mcp_connections::{ConnectionClient, Connections};
pub(crate) async fn run<C: ConnectionClient>(
    registry: &Connections<C>,
    response: &mut CommandOutput,
) {
    let servers = registry.list_servers().await;
    if servers.is_empty() {
        response.status = "No MCP servers connected".to_string();
        response.message = Some(("No MCP servers connected.").into());
    } else {
        response.status = format!("{} MCP server(s) connected", servers.len());
        let body = servers
            .into_iter()
            .map(|server| {
                format!(
                    "- {} ({} tools) :: {}",
                    server.name, server.tool_count, server.command
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        response.message = Some((format!("Connected MCP servers:\n{body}")).into());
    }
}
