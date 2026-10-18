//! CLI syntax for RalphArgs.
use clap::Parser;
use std::path::PathBuf;
/// Parsed options for the ralph args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::RalphArgs::try_parse_from(["ralph","status"]);
/// assert!(parsed.is_ok());
/// ```

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
