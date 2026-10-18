//! CLI syntax for ServeArgs.
use clap::Parser;
/// Parsed options for the serve args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::ServeArgs::try_parse_from(["command"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug)]
pub struct ServeArgs {
    /// Port to listen on
    #[arg(short, long, default_value = "4096")]
    pub port: u16,

    /// Hostname to bind to
    #[arg(long, default_value = "127.0.0.1")]
    pub hostname: String,

    /// Enable mDNS discovery
    #[arg(long)]
    pub mdns: bool,
}
