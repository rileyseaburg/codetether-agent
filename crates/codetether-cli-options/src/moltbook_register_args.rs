//! CLI syntax for MoltbookRegisterArgs.
use clap::Parser;
/// Parsed options for the moltbook register args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::MoltbookRegisterArgs::try_parse_from(["register","Agent"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug)]
pub struct MoltbookRegisterArgs {
    /// Agent name to register on Moltbook
    pub name: String,

    /// Optional extra description (CodeTether branding is always included)
    #[arg(short, long)]
    pub description: Option<String>,
}
