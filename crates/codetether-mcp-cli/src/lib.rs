//! MCP client command parsing, execution and presentation.
//!
//! Host provides built-in tool access and an authorized external client.
mod call;
mod connect;
mod list_tools;
mod reply;
mod resolve;
mod tools;
pub use resolve::resolve_command;
/// Native built-in tools and external-client binding.
/// # Implementors
/// Applications retain registry construction and tool authorization.
#[async_trait::async_trait(?Send)]
pub trait Host {
    /// Client whose connect and call operations enforce native policy.
    type Client: codetether_mcp_connections::ConnectionClient;
    /// Build the native local server and discover its tools.
    async fn builtin_tools() -> Vec<codetether_mcp_core::types::ToolMetadata>;
    /// Call one native local tool.
    /// # Errors
    /// Returns native tool setup or execution errors.
    async fn builtin_call(
        tool: &str,
        arguments: serde_json::Value,
    ) -> anyhow::Result<codetether_mcp_core::types::CallToolResult>;
}
/// Execute a client-side MCP operation using an already resolved command.
/// # Arguments
/// * `args` - Action and tool options.
/// * `resolved_command` - Optional subprocess executable and arguments.
/// # Returns
/// Unit after the requested operation and output complete.
/// # Errors
/// Returns command validation, native policy, client or serialization failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_mcp_cli::Host>(args: codetether_cli_options::McpArgs) {
/// let command = codetether_mcp_cli::resolve_command(&args);
/// let _ = codetether_mcp_cli::execute::<H>(args, command).await;
/// # }
/// ```
pub async fn execute<H: Host>(
    args: codetether_cli_options::McpArgs,
    resolved_command: Option<(String, Vec<String>)>,
) -> anyhow::Result<()> {
    match args.action.as_str() {
        "connect" => connect::execute::<H>(args, resolved_command).await,
        "list-tools" => list_tools::execute::<H>(args, resolved_command).await,
        "call" => call::execute::<H>(args, resolved_command).await,
        _ => anyhow::bail!(
            "Unknown action: {}. Use serve, connect, list-tools, or call",
            args.action
        ),
    }
}
