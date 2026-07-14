//! Top-level `codetether` subcommand enumeration.
//!
//! Split out of [`super`] so the module table stays within the file budget.

use super::{
    A2aArgs, AuthArgs, BenchmarkArgs, CleanupArgs, ContextArgs, ForageArgs,
    GitCredentialHelperArgs, IndexArgs, McpArgs, ModelsArgs, MoltbookArgs, OkrArgs, OracleArgs,
    PrArgs, RalphArgs, RlmArgs, RunArgs, SearchArgs, ServeArgs, SpawnArgs, StatsArgs, SwarmArgs,
    SwarmSubagentArgs, TuiArgs, WorktreeArgs, approval, browserctl, clipboard,
    config_args::ConfigArgs, connect,
};
use clap::Subcommand;

#[path = "mux_args.rs"]
pub mod mux_args;
use mux_args::MuxArgs;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start interactive terminal UI
    Tui(TuiArgs),

    /// Manage persistent network mux sessions
    Mux(MuxArgs),

    /// Start a headless API server
    Serve(ServeArgs),

    /// Run with a message (non-interactive)
    Run(RunArgs),

    /// Create or update GitHub pull requests with CodeTether provenance
    Pr(PrArgs),

    /// Authenticate provider credentials and store in Vault
    Auth(AuthArgs),

    /// SSH into an Ubuntu VM and run a device-code auth, opening the URL locally
    Connect(connect::ConnectArgs),

    /// Review and decide pending approval requests
    Approval(approval::ApprovalArgs),

    /// Manage configuration
    Config(ConfigArgs),

    /// Convert clipboard images into terminal-pasteable CodeTether input
    Clipboard(clipboard::ClipboardArgs),

    /// Browse or reset the active session context
    Context(ContextArgs),

    /// A2A worker mode (explicit - also the default)
    Worker(A2aArgs),

    /// Internal command: Git credential helper for worker-managed repositories.
    #[command(hide = true)]
    GitCredentialHelper(GitCredentialHelperArgs),

    /// Spawn an A2A agent runtime with auto card registration and peer discovery
    Spawn(SpawnArgs),

    /// Execute task with parallel sub-agents (swarm mode)
    Swarm(SwarmArgs),

    /// Internal command: execute one swarm subtask payload.
    #[command(hide = true)]
    SwarmSubagent(SwarmSubagentArgs),

    /// Analyze large content with RLM (Recursive Language Model)
    Rlm(RlmArgs),

    /// Deterministic oracle utilities (validate/sync)
    Oracle(OracleArgs),

    /// Autonomous PRD-driven agent loop (Ralph)
    Ralph(RalphArgs),

    /// Model Context Protocol (MCP) server/client
    Mcp(McpArgs),

    /// Show telemetry and execution statistics
    Stats(StatsArgs),

    /// Clean up orphaned worktrees and branches from failed Ralph runs
    Cleanup(CleanupArgs),

    /// Manage git worktrees and VS Code integration
    Worktree(WorktreeArgs),

    /// List available models from all configured providers
    Models(ModelsArgs),

    /// Build a persistent codebase index for faster workspace introspection
    Index(IndexArgs),

    /// Run benchmark suite against models using Ralph PRDs
    Benchmark(BenchmarkArgs),

    /// Moltbook — social network for AI agents
    Moltbook(MoltbookArgs),

    /// Manage OKRs (Objectives and Key Results)
    Okr(OkrArgs),

    /// OKR-governed autonomous opportunity scanner/executor
    Forage(ForageArgs),

    /// LLM-routed search across grep/glob/web/memory/RLM backends
    Search(SearchArgs),

    /// Control a local Chromium browser via DevTools (alias: browser)
    #[command(name = "browserctl", visible_alias = "browser")]
    Browserctl(browserctl::BrowserCtlArgs),
}
