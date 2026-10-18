//! CLI syntax for MoltbookSearchArgs.
use clap::Parser;
/// Parsed options for the moltbook search args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::MoltbookSearchArgs::try_parse_from(["search","Text"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug)]
pub struct MoltbookSearchArgs {
    /// Search query
    pub query: String,

    /// Max results
    #[arg(short, long, default_value = "10")]
    pub limit: usize,
}
