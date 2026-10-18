//! CLI syntax for PrArgs.
use crate::PrCommand;
use clap::Parser;
/// Parsed options for the pr args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::PrArgs::try_parse_from(["pr","create","--title","Example"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug)]
pub struct PrArgs {
    #[command(subcommand)]
    pub command: PrCommand,
}
