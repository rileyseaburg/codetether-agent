//! MCP call command with native host boundaries.
use codetether_cli_options::McpArgs;
use codetether_mcp_connections::ConnectionClient;
use codetether_mcp_core::registry::RegistryClient;
pub(super) async fn execute<H: crate::Host>(
    args: McpArgs,
    resolved_command: Option<(String, Vec<String>)>,
) -> anyhow::Result<()> {
    let tool = args
        .tool
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("--tool required for call action"))?;
    let arguments: serde_json::Value = args
        .arguments
        .as_ref()
        .map(|s| serde_json::from_str(s))
        .transpose()?
        .unwrap_or(serde_json::json!({}));

    if let Some((cmd, cmd_args)) = resolved_command.as_ref() {
        // Call tool on external MCP server
        let cmd_arg_refs: Vec<&str> = cmd_args.iter().map(String::as_str).collect();
        let client = H::Client::connect_subprocess(cmd, &cmd_arg_refs).await?;
        let result = client.call_tool(tool, arguments).await?;
        client.close().await?;

        crate::reply::print(&result);
    } else {
        // Call built-in tool directly via MCP server
        let result = H::builtin_call(tool, arguments).await?;

        crate::reply::print(&result);
    }
    Ok(())
}
