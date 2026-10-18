//! CLI syntax for MoltbookUpdateProfileArgs.
use clap::Parser;
/// Parsed options for the moltbook update profile args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::MoltbookUpdateProfileArgs::try_parse_from(["command"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug)]
pub struct MoltbookUpdateProfileArgs {
    /// Extra description to append
    #[arg(short, long)]
    pub description: Option<String>,
}
