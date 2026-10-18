//! Invoke a parsed tool call and construct its response.
use crate::CommandOutput;
use codetether_mcp_connections::{ConnectionClient, Connections};
use serde_json::Value;
pub(crate) async fn run<C: ConnectionClient>(
    registry: &Connections<C>,
    server_name: &str,
    tool_name: &str,
    arguments: Value,
    response: &mut CommandOutput,
) {
    match registry.call_tool(server_name, tool_name, arguments).await {
        Ok(output) => {
            response.status = format!("MCP tool finished: {server_name}/{tool_name}");
            response.message =
                Some((format!("MCP `{server_name}` / `{tool_name}` result:\n{output}")).into());
        }
        Err(error) => {
            response.status = format!("MCP call failed: {error}");
            response.message =
                Some((format!("MCP `{server_name}` / `{tool_name}` failed: {error}")).into());
        }
    }
}
