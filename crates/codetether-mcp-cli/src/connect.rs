//! MCP connect command with native host boundaries.
use codetether_cli_options::McpArgs;
use codetether_mcp_connections::ConnectionClient;
use codetether_mcp_core::registry::RegistryClient;
pub(super) async fn execute<H: crate::Host>(
    args: McpArgs,
    resolved_command: Option<(String, Vec<String>)>,
) -> anyhow::Result<()> {
    let (cmd, cmd_args) = resolved_command.as_ref().ok_or_else(|| {
                        anyhow::anyhow!(
                            "command required for connect action (pass it positionally, e.g. `codetether mcp connect npx -y @modelcontextprotocol/server-filesystem /path`, or via --command)"
                        )
                    })?;
    let cmd_arg_refs: Vec<&str> = cmd_args.iter().map(String::as_str).collect();

    tracing::info!("Connecting to MCP server: {} {}", cmd, cmd_args.join(" "));
    let client = H::Client::connect_subprocess(cmd, &cmd_arg_refs).await?;

    // List available tools
    let tools = client.tools().await;

    crate::tools::connected(&tools, args.json)?;
    // Keep connection alive briefly to show it works
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    client.close().await?;

    Ok(())
}
