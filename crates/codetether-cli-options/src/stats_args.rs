//! CLI syntax for StatsArgs.
use clap::Parser;
/// Parsed options for the stats args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::StatsArgs::try_parse_from(["command"]);
/// assert!(parsed.is_ok());
/// ```

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
