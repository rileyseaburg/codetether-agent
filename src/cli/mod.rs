//! CLI command definitions and handlers

pub mod config;
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

    /// Manage configuration
    Config(ConfigArgs),

    /// A2A worker mode (explicit - also the default)
    Worker(A2aArgs),

    /// Execute task with parallel sub-agents (swarm mode)
    Swarm(SwarmArgs),
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

#[derive(Parser, Debug, Clone)]
pub struct A2aArgs {
    /// A2A server URL
    #[arg(short, long, env = "CODETETHER_SERVER")]
    pub server: String,

    /// Authentication token
    #[arg(short, long, env = "CODETETHER_TOKEN")]
    pub token: Option<String>,

    /// Worker name
    #[arg(short, long, env = "CODETETHER_WORKER_NAME")]
    pub name: Option<String>,

    /// Comma-separated list of codebase paths
    #[arg(short, long)]
    pub codebases: Option<String>,

    /// Auto-approve policy: all, safe (read-only), none
    #[arg(long, default_value = "safe", value_parser = ["all", "safe", "none"])]
    pub auto_approve: String,

    /// Email for task completion reports
    #[arg(short, long)]
    pub email: Option<String>,

    /// Push notification endpoint URL
    #[arg(long)]
    pub push_url: Option<String>,
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

    /// Decomposition strategy: auto, domain, data, stage, none
    #[arg(short, long, default_value = "auto")]
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
}
