//! CLI syntax for MoltbookArgs.
use crate::MoltbookCommand;
use clap::Parser;
/// Parsed options for the moltbook args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::MoltbookArgs::try_parse_from(["moltbook","status"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug)]
pub struct MoltbookArgs {
    #[command(subcommand)]
    pub command: MoltbookCommand,
}
