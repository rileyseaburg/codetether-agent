//! MCP list-tools command with native host boundaries.
use codetether_cli_options::McpArgs;
use codetether_mcp_connections::ConnectionClient;
use codetether_mcp_core::registry::RegistryClient;
pub(super) async fn execute<H: crate::Host>(
    args: McpArgs,
    resolved_command: Option<(String, Vec<String>)>,
) -> anyhow::Result<()> {
    if let Some((cmd, cmd_args)) = resolved_command.as_ref() {
        // Connect to external MCP server and list its tools
        let cmd_arg_refs: Vec<&str> = cmd_args.iter().map(String::as_str).collect();
        let client = H::Client::connect_subprocess(cmd, &cmd_arg_refs).await?;
        let tools = client.tools().await;
        crate::tools::available(&tools, args.json)?;
        client.close().await?;
    } else {
        // List built-in tools from the MCP server registry
        let tools = H::builtin_tools().await;
        crate::tools::available(&tools, args.json)?;
    }
    Ok(())
}
