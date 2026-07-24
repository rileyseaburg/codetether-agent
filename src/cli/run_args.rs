use crate::config::AccessMode;
use clap::Parser;
use std::path::PathBuf;

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
    /// Transient access mode: ask, approve, or full
    #[arg(long, value_parser = clap::value_parser!(AccessMode))]
    pub access_mode: Option<AccessMode>,
    /// Full-auto mode: skip all prompts, auto-apply edits, no sandbox.
    /// Equivalent to --access-mode full with edit auto-apply on.
    #[arg(long)]
    pub yolo: bool,
    /// Output format
    #[arg(long, default_value = "default", value_parser = ["default", "json", "jsonl"])]
    pub format: String,
    /// Files to attach
    #[arg(short, long)]
    pub file: Vec<PathBuf>,
    /// Import and continue a Codex CLI session by ID
    #[arg(long)]
    pub codex_session: Option<String>,
    /// Maximum agentic loop steps (unbounded when omitted, minimum: 1)
    #[arg(long)]
    pub max_steps: Option<usize>,
    /// Auto-continue checkpoint/resume cycles until this many attempts
    #[arg(long)]
    pub auto_continue_until: Option<usize>,
    /// Number of parallel speculative branches to race (1-8, default: 1).
    #[arg(long, default_value = "1")]
    pub branches: usize,
    /// Optional comma-separated strategy prompts for speculative branches.
    #[arg(long, value_delimiter = ',')]
    pub strategies: Vec<String>,
    /// Use tool-free GPT-5.6 Sol planning and LSP review around a coding worker.
    #[arg(long)]
    pub sol_planner: bool,
}
