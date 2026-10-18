//! CLI syntax for MoltbookPostArgs.
use clap::Parser;
/// Parsed options for the moltbook post args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::MoltbookPostArgs::try_parse_from(["post","Title","--content","Body"]);
/// assert!(parsed.is_ok());
/// ```

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
