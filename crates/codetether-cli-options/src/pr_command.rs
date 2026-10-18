//! CLI syntax for PrCommand.
use crate::CreatePrArgs;
use clap::Subcommand;
/// Parsed options for the pr command command.
/// # Examples
/// ```
/// use clap::Parser;
/// use codetether_cli_options::{PrCommand, CreatePrArgs};
/// let args = CreatePrArgs::parse_from(["pr", "--title", "Example"]);
/// let PrCommand::Create(args) = PrCommand::Create(args);
/// assert_eq!(args.title, "Example");
/// ```

#[derive(Subcommand, Debug)]
pub enum PrCommand {
    /// Create or update a pull request for the current branch
    Create(CreatePrArgs),
}
