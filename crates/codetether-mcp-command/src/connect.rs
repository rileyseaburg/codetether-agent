//! Handle the MCP connect subcommand.
use crate::CommandOutput;
use codetether_mcp_connections::{ConnectionClient, Connections};
pub(crate) async fn run<C: ConnectionClient>(
    registry: &Connections<C>,
    value: &str,
    response: &mut CommandOutput,
) {
    let mut parts = value.trim().splitn(2, char::is_whitespace);
    let Some(name) = parts.next().filter(|part| !part.is_empty()) else {
        response.status = "Usage: /mcp connect <name> <command...>".to_string();
        return;
    };
    let Some(command) = parts.next().map(str::trim).filter(|part| !part.is_empty()) else {
        response.status = "Usage: /mcp connect <name> <command...>".to_string();
        return;
    };

    match registry.connect(name, command).await {
        Ok(tool_count) => {
            response.status = format!("Connected MCP server '{name}' ({tool_count} tools)");
            response.message =
                Some((format!("Connected MCP server `{name}` with {tool_count} tools.")).into());
        }
        Err(error) => {
            response.status = format!("MCP connect failed: {error}");
            response.message = Some((format!("MCP connect failed for `{name}`: {error}")).into());
        }
    }
}
