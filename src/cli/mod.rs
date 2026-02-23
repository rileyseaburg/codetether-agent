//! CLI command definitions and handlers

pub mod auth;
pub mod config;
pub mod go_ralph;
pub mod run;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// CodeTether Agent - A2A-native AI coding agent
///
/// By default, runs as an A2A worker connecting to the CodeTether server.
/// Use the 'tui' subcommand for interactive terminal mode.
#[derive(Parser, Debug)]
#[command(name = "codetether")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Project directory to operate on
    #[arg(global = true)]
    pub project: Option<PathBuf>,

    /// Print logs to stderr
    #[arg(long, global = true)]
    pub print_logs: bool,

    /// Log level
    #[arg(long, global = true, value_parser = ["DEBUG", "INFO", "WARN", "ERROR"])]
    pub log_level: Option<String>,

    // Default A2A args (when no subcommand)
    /// A2A server URL (default mode)
    #[arg(short, long, env = "CODETETHER_SERVER")]
    pub server: Option<String>,

    /// Authentication token
    #[arg(short, long, env = "CODETETHER_TOKEN")]
    pub token: Option<String>,

    /// Worker name
    #[arg(short, long, env = "CODETETHER_WORKER_NAME")]
    pub name: Option<String>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start interactive terminal UI
    Tui(TuiArgs),

    /// Start a headless API server
    Serve(ServeArgs),

    /// Run with a message (non-interactive)
    Run(RunArgs),

    /// Authenticate provider credentials and store in Vault
    Auth(AuthArgs),

    /// Manage configuration
    Config(ConfigArgs),

    /// A2A worker mode (explicit - also the default)
    Worker(A2aArgs),

    /// Spawn an A2A agent runtime with auto card registration and peer discovery
    Spawn(SpawnArgs),

    /// Execute task with parallel sub-agents (swarm mode)
    Swarm(SwarmArgs),

    /// Internal command: execute one swarm subtask payload.
    #[command(hide = true)]
    SwarmSubagent(SwarmSubagentArgs),

    /// Analyze large content with RLM (Recursive Language Model)
    Rlm(RlmArgs),

    /// Autonomous PRD-driven agent loop (Ralph)
    Ralph(RalphArgs),

    /// Model Context Protocol (MCP) server/client
    Mcp(McpArgs),

    /// Show telemetry and execution statistics
    Stats(StatsArgs),

    /// Clean up orphaned worktrees and branches from failed Ralph runs
    Cleanup(CleanupArgs),

    /// List available models from all configured providers
    Models(ModelsArgs),

    /// Run benchmark suite against models using Ralph PRDs
    Benchmark(BenchmarkArgs),

    /// Moltbook — social network for AI agents
    Moltbook(MoltbookArgs),

    /// Manage OKRs (Objectives and Key Results)
    Okr(OkrArgs),
}

#[derive(Parser, Debug)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommand,
}

#[derive(Subcommand, Debug)]
pub enum AuthCommand {
    /// Authenticate with GitHub Copilot using device flow
    Copilot(CopilotAuthArgs),

    /// Register a new CodeTether account with email/password
    Register(RegisterAuthArgs),

    /// Login to a CodeTether server with email/password
    Login(LoginAuthArgs),
}

#[derive(Parser, Debug)]
pub struct RegisterAuthArgs {
    /// CodeTether server URL (e.g., https://api.codetether.run)
    #[arg(short, long, env = "CODETETHER_SERVER")]
    pub server: String,

    /// Email address
    #[arg(short, long)]
    pub email: Option<String>,

    /// First name (optional)
    #[arg(long)]
    pub first_name: Option<String>,

    /// Last name (optional)
    #[arg(long)]
    pub last_name: Option<String>,

    /// Referral source (optional)
    #[arg(long)]
    pub referral_source: Option<String>,
}

#[derive(Parser, Debug)]
pub struct LoginAuthArgs {
    /// CodeTether server URL (e.g., https://api.codetether.io)
    #[arg(short, long, env = "CODETETHER_SERVER")]
    pub server: String,

    /// Email address
    #[arg(short, long)]
    pub email: Option<String>,
}

#[derive(Parser, Debug)]
pub struct CopilotAuthArgs {
    /// GitHub Enterprise URL or domain (e.g. company.ghe.com)
    #[arg(long)]
    pub enterprise_url: Option<String>,

    /// GitHub OAuth app client ID for Copilot device flow
    #[arg(long, env = "CODETETHER_COPILOT_OAUTH_CLIENT_ID")]
    pub client_id: Option<String>,
}

#[derive(Parser, Debug)]
pub struct TuiArgs {
    /// Project directory
    pub project: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct ServeArgs {
    /// Port to listen on
    #[arg(short, long, default_value = "4096")]
    pub port: u16,

    /// Hostname to bind to
    #[arg(long, default_value = "127.0.0.1")]
    pub hostname: String,

    /// Enable mDNS discovery
    #[arg(long)]
    pub mdns: bool,
}

#[derive(Parser, Debug)]
pub struct RunArgs {
    /// Message to send (can be multiple words, quoted or unquoted)
    pub message: String,

    /// Continue the last session
    #[arg(short, long)]
    pub continue_session: bool,

    /// Session ID to continue
    #[arg(short, long)]
    pub session: Option<String>,

    /// Model to use (provider/model format)
    #[arg(short, long)]
    pub model: Option<String>,

    /// Agent to use
    #[arg(long)]
    pub agent: Option<String>,

    /// Output format
    #[arg(long, default_value = "default", value_parser = ["default", "json"])]
    pub format: String,

    /// Files to attach
    #[arg(short, long)]
    pub file: Vec<PathBuf>,
}

/// Arguments for standalone worker HTTP server (testing/debugging)
#[derive(Parser, Debug, Clone)]
pub struct WorkerServerArgs {
    /// Hostname to bind
    #[arg(long, default_value = "0.0.0.0")]
    pub hostname: String,

    /// Port to bind
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}

#[derive(Parser, Debug, Clone)]
pub struct A2aArgs {
    /// A2A server URL
    #[arg(short, long, env = "CODETETHER_SERVER", default_value = crate::a2a::worker::DEFAULT_A2A_SERVER_URL)]
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

    /// Disable the worker HTTP server (for environments without K8s)
    #[arg(long, env = "CODETETHER_WORKER_HTTP_DISABLED")]
    pub no_http_server: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct SpawnArgs {
    /// Agent name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Hostname to bind the spawned A2A agent
    #[arg(long, default_value = "127.0.0.1")]
    pub hostname: String,

    /// Port to bind the spawned A2A agent
    #[arg(short, long, default_value = "4097")]
    pub port: u16,

    /// Public URL published in the agent card (defaults to http://<hostname>:<port>)
    #[arg(long)]
    pub public_url: Option<String>,

    /// Optional custom agent description for the card
    #[arg(short, long)]
    pub description: Option<String>,

    /// Peer seed URLs to discover and talk to (repeat flag or comma-separated)
    #[arg(long, value_delimiter = ',', env = "CODETETHER_A2A_PEERS")]
    pub peer: Vec<String>,

    /// Discovery interval in seconds
    #[arg(long, default_value = "15")]
    pub discovery_interval_secs: u64,

    /// Disable sending an automatic intro message to newly discovered peers
    #[arg(long = "no-auto-introduce", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub auto_introduce: bool,
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    /// Show current configuration
    #[arg(long)]
    pub show: bool,

    /// Initialize default configuration
    #[arg(long)]
    pub init: bool,

    /// Set a configuration value
    #[arg(long)]
    pub set: Option<String>,
}

#[derive(Parser, Debug)]
pub struct SwarmArgs {
    /// Task to execute with swarm
    pub task: String,

    /// Model to use (provider/model format, e.g. zai/glm-5 or openrouter/z-ai/glm-5)
    #[arg(short, long)]
    pub model: Option<String>,

    /// Decomposition strategy: auto, domain, data, stage, none
    #[arg(short = 's', long, default_value = "auto")]
    pub strategy: String,

    /// Maximum number of concurrent sub-agents
    #[arg(long, default_value = "100")]
    pub max_subagents: usize,

    /// Maximum steps per sub-agent
    #[arg(long, default_value = "100")]
    pub max_steps: usize,

    /// Timeout per sub-agent (seconds)
    #[arg(long, default_value = "300")]
    pub timeout: u64,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Sub-agent execution mode: local | k8s
    #[arg(long, default_value = "local", value_parser = ["local", "k8s", "kubernetes", "kubernetes-pod", "pod"])]
    pub execution_mode: String,

    /// Maximum concurrent Kubernetes sub-agent pods when execution mode is k8s.
    #[arg(long, default_value = "8")]
    pub k8s_pod_budget: usize,

    /// Optional image override for Kubernetes sub-agent pods.
    #[arg(long)]
    pub k8s_image: Option<String>,
}

#[derive(Parser, Debug)]
pub struct SwarmSubagentArgs {
    /// Base64 payload for the remote subtask (JSON encoded)
    #[arg(long)]
    pub payload_base64: Option<String>,

    /// Read payload from an environment variable
    #[arg(long, default_value = "CODETETHER_SWARM_SUBTASK_PAYLOAD")]
    pub payload_env: String,
}

#[derive(Parser, Debug)]
pub struct RlmArgs {
    /// Query to answer about the content
    pub query: String,

    /// File paths to analyze
    #[arg(short, long)]
    pub file: Vec<PathBuf>,

    /// Direct content to analyze (use - for stdin)
    #[arg(long)]
    pub content: Option<String>,

    /// Content type hint: code, logs, conversation, documents, auto
    #[arg(long, default_value = "auto")]
    pub content_type: String,

    /// Maximum tokens for output
    #[arg(long, default_value = "4000")]
    pub max_tokens: usize,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Enable verbose output (shows context summary)
    #[arg(short, long)]
    pub verbose: bool,

    /// Validate FINAL payload with deterministic oracle when possible
    #[arg(long)]
    pub oracle_verify: bool,

    /// Number of independent runs for semantic consensus verification
    #[arg(long, default_value = "1")]
    pub consensus_runs: usize,

    /// Consensus threshold for semantic verification (1.0 = unanimous)
    #[arg(long, default_value = "1.0")]
    pub consensus_threshold: f32,

    /// Directory to write split oracle JSONL datasets
    #[arg(long)]
    pub oracle_out_dir: Option<PathBuf>,

    /// Output prefix for oracle split JSONL files
    #[arg(long, default_value = "rlm_oracle")]
    pub oracle_prefix: String,
}

#[derive(Parser, Debug)]
pub struct RalphArgs {
    /// Action to perform
    #[arg(value_parser = ["run", "status", "create-prd"])]
    pub action: String,

    /// Path to prd.json file
    #[arg(short, long, default_value = "prd.json")]
    pub prd: PathBuf,

    /// Feature name (for create-prd)
    #[arg(short, long)]
    pub feature: Option<String>,

    /// Project name (for create-prd)
    #[arg(long = "project-name")]
    pub project_name: Option<String>,

    /// Maximum iterations
    #[arg(long, default_value = "10")]
    pub max_iterations: usize,

    /// Model to use
    #[arg(short, long)]
    pub model: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct McpArgs {
    /// Action to perform
    #[arg(value_parser = ["serve", "connect", "list-tools", "call"])]
    pub action: String,

    /// Command to spawn for connecting to MCP server
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

    /// URL of the CodeTether HTTP server's bus SSE endpoint.
    /// When set, the MCP server connects to the agent bus and exposes
    /// bus_events, bus_status, and ralph_status tools plus codetether:// resources.
    /// Example: http://localhost:8001/v1/bus/stream
    #[arg(long)]
    pub bus_url: Option<String>,
}

#[derive(Parser, Debug)]
pub struct StatsArgs {
    /// Show tool execution history
    #[arg(short, long)]
    pub tools: bool,

    /// Show file change history
    #[arg(short, long)]
    pub files: bool,

    /// Show token usage
    #[arg(long)]
    pub tokens: bool,

    /// Filter by tool name
    #[arg(long)]
    pub tool: Option<String>,

    /// Filter by file path
    #[arg(long)]
    pub file: Option<String>,

    /// Number of recent entries to show
    #[arg(short, long, default_value = "20")]
    pub limit: usize,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Show all/summary (default shows summary)
    #[arg(long)]
    pub all: bool,
}

#[derive(Parser, Debug)]
pub struct CleanupArgs {
    /// Dry run - show what would be cleaned up without deleting
    #[arg(short, long)]
    pub dry_run: bool,

    /// Clean up worktrees only (not branches)
    #[arg(long)]
    pub worktrees_only: bool,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct ModelsArgs {
    /// Filter by provider name
    #[arg(short, long)]
    pub provider: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct MoltbookArgs {
    #[command(subcommand)]
    pub command: MoltbookCommand,
}

#[derive(Subcommand, Debug)]
pub enum MoltbookCommand {
    /// Register a new agent on Moltbook
    Register(MoltbookRegisterArgs),

    /// Check claim status
    Status,

    /// View your Moltbook profile
    Profile,

    /// Update your profile description
    UpdateProfile(MoltbookUpdateProfileArgs),

    /// Create a post (defaults to m/general)
    Post(MoltbookPostArgs),

    /// Post a CodeTether introduction to Moltbook
    Intro,

    /// Run a heartbeat — check feed, show recent posts
    Heartbeat,

    /// Comment on a Moltbook post
    Comment(MoltbookCommentArgs),

    /// Search Moltbook posts and comments
    Search(MoltbookSearchArgs),
}

#[derive(Parser, Debug)]
pub struct MoltbookRegisterArgs {
    /// Agent name to register on Moltbook
    pub name: String,

    /// Optional extra description (CodeTether branding is always included)
    #[arg(short, long)]
    pub description: Option<String>,
}

#[derive(Parser, Debug)]
pub struct MoltbookUpdateProfileArgs {
    /// Extra description to append
    #[arg(short, long)]
    pub description: Option<String>,
}

#[derive(Parser, Debug)]
pub struct MoltbookPostArgs {
    /// Post title
    pub title: String,

    /// Post content
    #[arg(short, long)]
    pub content: String,

    /// Submolt to post in
    #[arg(short, long, default_value = "general")]
    pub submolt: String,
}

#[derive(Parser, Debug)]
pub struct MoltbookCommentArgs {
    /// Post ID to comment on
    pub post_id: String,

    /// Comment content
    pub content: String,
}

#[derive(Parser, Debug)]
pub struct MoltbookSearchArgs {
    /// Search query
    pub query: String,

    /// Max results
    #[arg(short, long, default_value = "10")]
    pub limit: usize,
}

#[derive(Parser, Debug)]
pub struct BenchmarkArgs {
    /// Directory containing benchmark PRD files
    #[arg(long, default_value = "benchmarks")]
    pub prd_dir: String,

    /// Models to benchmark (comma-separated, format: provider:model)
    #[arg(short, long, value_delimiter = ',')]
    pub models: Vec<String>,

    /// Only run PRDs matching this tier (1, 2, or 3)
    #[arg(long)]
    pub tier: Option<u8>,

    /// Run model×PRD combos in parallel
    #[arg(long)]
    pub parallel: bool,

    /// Maximum iterations per story
    #[arg(long, default_value = "10")]
    pub max_iterations: usize,

    /// Timeout per story in seconds
    #[arg(long, default_value = "300")]
    pub story_timeout: u64,

    /// Output file path
    #[arg(short, long, default_value = "benchmark_results.json")]
    pub output: String,

    /// Cost ceiling per run in USD (prevents runaway spending)
    #[arg(long, default_value = "50.0")]
    pub cost_ceiling: f64,

    /// Submit results to this API URL
    #[arg(long)]
    pub submit_url: Option<String>,

    /// API key for submitting results (Bearer token)
    #[arg(long, env = "BENCHMARK_API_KEY")]
    pub submit_key: Option<String>,

    /// Output as JSON to stdout
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct OkrArgs {
    /// Action to perform
    #[arg(value_parser = ["list", "status", "create", "runs", "export", "stats", "report"])]
    pub action: String,

    /// OKR ID (UUID)
    #[arg(short, long)]
    pub id: Option<String>,

    /// OKR title (for create)
    #[arg(short, long)]
    pub title: Option<String>,

    /// OKR description (for create)
    #[arg(short, long)]
    pub description: Option<String>,

    /// Target value for key result (for create)
    #[arg(long)]
    pub target: Option<f64>,

    /// Unit for key result (for create)
    #[arg(long, default_value = "%")]
    pub unit: String,

    /// Filter by status: draft, active, completed, cancelled, on_hold
    #[arg(long)]
    pub status: Option<String>,

    /// Filter by owner
    #[arg(long)]
    pub owner: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Include evidence links in output
    #[arg(long)]
    pub evidence: bool,
}
