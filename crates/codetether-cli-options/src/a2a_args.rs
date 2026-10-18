//! CLI syntax for A2aArgs.
use clap::Parser;
/// Parsed options for the a2a args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::A2aArgs::try_parse_from(["command"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug, Clone)]
pub struct A2aArgs {
    /// A2A server URL
    #[arg(short, long, env = "CODETETHER_SERVER", default_value = codetether_a2a_worker_core::DEFAULT_A2A_SERVER_URL)]
    pub server: String,

    /// Authentication token
    #[arg(short, long, env = "CODETETHER_TOKEN")]
    pub token: Option<String>,

    /// Worker name
    #[arg(short, long, env = "CODETETHER_WORKER_NAME")]
    pub name: Option<String>,

    /// Comma-separated list of workspace paths (alias: --codebases)
    #[arg(short, long, visible_alias = "codebases")]
    pub workspaces: Option<String>,

    /// Maximum number of tasks this worker will process concurrently
    #[arg(
        long,
        env = "CODETETHER_WORKER_MAX_CONCURRENT_TASKS",
        default_value_t = 4
    )]
    pub max_concurrent_tasks: usize,

    /// Auto-approve policy: all, safe (read-only), none
    #[arg(long, default_value = "safe", value_parser = ["all", "safe", "none"])]
    pub auto_approve: String,

    /// Email for task completion reports
    #[arg(short, long)]
    pub email: Option<String>,

    /// Push notification endpoint URL
    #[arg(long)]
    pub push_url: Option<String>,

    /// Hostname to bind the worker HTTP server
    #[arg(long, default_value = "0.0.0.0", env = "CODETETHER_WORKER_HOST")]
    pub hostname: String,

    /// Port for the worker HTTP server (for health/readiness probes)
    #[arg(long, default_value = "8080", env = "CODETETHER_WORKER_PORT")]
    pub port: u16,

    /// Public base URL advertised for this worker's HTTP interfaces.
    #[arg(long, env = "CODETETHER_WORKER_PUBLIC_URL")]
    pub public_url: Option<String>,

    /// Disable the worker HTTP server (for environments without K8s)
    #[arg(long, env = "CODETETHER_WORKER_HTTP_DISABLED")]
    pub no_http_server: bool,
}
