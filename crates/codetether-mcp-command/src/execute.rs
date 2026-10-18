//! Dispatch MCP command text while preserving existing prefix semantics.
use crate::CommandOutput;
use codetether_mcp_connections::{ConnectionClient, Connections};
/// Parse and execute one MCP subcommand through the supplied registry.
/// # Arguments
/// * `registry` - Registry with caller-authorized clients.
/// * `raw` - Text following the MCP command name.
/// # Returns
/// A status line and optional system message, including error/usage responses.
/// # Examples
/// ```no_run
/// # async fn example<C: codetether_mcp_connections::ConnectionClient>(registry: &codetether_mcp_connections::Connections<C>) {
/// let result = codetether_mcp_command::execute(registry, "servers").await;
/// assert!(!result.status.is_empty());
/// # }
/// ```
pub async fn execute<C: ConnectionClient>(registry: &Connections<C>, raw: &str) -> CommandOutput {
    let rest = raw.trim();
    let mut output = CommandOutput::default();
    if let Some(value) = rest.strip_prefix("connect ") {
        crate::connect::run(registry, value, &mut output).await;
    } else if rest == "servers" {
        crate::servers::run(registry, &mut output).await;
    } else if let Some(value) = rest.strip_prefix("tools") {
        crate::tools::run(registry, value, &mut output).await;
    } else if let Some(value) = rest.strip_prefix("call ") {
        crate::call::run(registry, value, &mut output).await;
    } else {
        output.status = "Usage: /mcp connect <name> <command...> | /mcp servers | /mcp tools [server] | /mcp call <server> <tool> [json]".to_string();
    }
    output
}
