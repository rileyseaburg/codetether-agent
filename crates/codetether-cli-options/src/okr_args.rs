//! CLI syntax for OkrArgs.
use clap::Parser;
/// Parsed options for the okr args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::OkrArgs::try_parse_from(["okr","list"]);
/// assert!(parsed.is_ok());
/// ```

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
