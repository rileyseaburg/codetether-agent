//! CLI syntax for RlmArgs.
use clap::Parser;
use std::path::PathBuf;
/// Parsed options for the rlm args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::RlmArgs::try_parse_from(["rlm","Question"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug)]
pub struct RlmArgs {
    /// Query to answer about the content
    pub query: String,

    /// Model to use (provider/model format)
    #[arg(short, long)]
    pub model: Option<String>,

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

    /// Deprecated no-op: oracle verification is now always enabled by default
    #[arg(long)]
    pub oracle_verify: bool,

    /// Disable deterministic oracle verification (emergency opt-out)
    #[arg(long)]
    pub no_oracle_verify: bool,

    /// Number of independent runs for semantic consensus verification
    #[arg(long, default_value = "1")]
    pub consensus_runs: usize,

    /// Consensus threshold for semantic verification (1.0 = unanimous)
    #[arg(long, default_value = "1.0")]
    pub consensus_threshold: f32,

    /// Temperature for RLM generation (defaults: 0.3 single run, 0.75 consensus)
    #[arg(long)]
    pub analysis_temperature: Option<f32>,

    /// Directory to write split oracle JSONL datasets
    #[arg(long)]
    pub oracle_out_dir: Option<PathBuf>,

    /// Output prefix for oracle split JSONL files
    #[arg(long, default_value = "rlm_oracle")]
    pub oracle_prefix: String,
}
