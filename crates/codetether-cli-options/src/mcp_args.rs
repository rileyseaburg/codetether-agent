//! CLI syntax for McpArgs.
use clap::Parser;
/// Parsed options for the mcp args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::McpArgs::try_parse_from(["mcp","list-tools"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug)]
pub struct McpArgs {
    /// Action to perform
    #[arg(value_parser = ["serve", "connect", "list-tools", "call"])]
    pub action: String,

    /// Command to spawn for connecting to MCP server (flag form)
    #[arg(short, long)]
    pub command: Option<String>,

    /// Server name for registry
    #[arg(long)]
    pub server_name: Option<String>,

    /// Tool name for call action
    #[arg(long)]
    pub tool: Option<String>,

    /// JSON arguments for tool call
    #[arg(long)]
    pub arguments: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Resolve a worker's first-class bus connection via the control plane.
    #[arg(long)]
    pub worker_id: Option<String>,

    /// Resolve the owning worker via a registered workspace ID.
    #[arg(long)]
    pub workspace_id: Option<String>,

    /// URL of the CodeTether HTTP server's bus SSE endpoint.
    /// When set, the MCP server connects to the agent bus and exposes
    /// bus_events, bus_status, and ralph_status tools plus codetether:// resources.
    /// Example: http://localhost:8001/v1/bus/stream
    #[arg(long)]
    pub bus_url: Option<String>,

    /// Command (and args) to spawn for connecting to MCP server (positional form).
    /// Example: `codetether mcp connect npx -y @modelcontextprotocol/server-filesystem /path`
    #[arg(allow_hyphen_values = true)]
    pub command_args: Vec<String>,
}
