//! CLI syntax for CreatePrArgs.
use clap::Parser;
use std::path::PathBuf;
/// Parsed options for the create pr args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::CreatePrArgs::try_parse_from(["pr","--title","Example"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug, Clone)]
pub struct CreatePrArgs {
    /// Pull request title
    #[arg(long)]
    pub title: String,

    /// Optional pull request body
    #[arg(long)]
    pub body: Option<String>,

    /// Optional path to a pull request body file
    #[arg(long)]
    pub body_file: Option<PathBuf>,

    /// Base branch
    #[arg(long, default_value = "main")]
    pub base: String,

    /// Head branch override (defaults to current branch)
    #[arg(long)]
    pub head: Option<String>,

    /// Create the PR as draft
    #[arg(long, default_value_t = false)]
    pub draft: bool,

    /// Emit JSON instead of KEY=VALUE lines
    #[arg(long, default_value_t = false)]
    pub json: bool,

    /// Project directory override
    #[arg(long)]
    pub project: Option<PathBuf>,
}
