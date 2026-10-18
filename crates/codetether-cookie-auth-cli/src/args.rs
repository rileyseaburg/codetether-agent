//! Provider cookie import arguments.
use clap::Parser;
use std::path::PathBuf;

/// Select an exported cookie file and provider-specific filtering policy.
///
/// # Examples
/// ```
/// use clap::Parser;
/// let args = codetether_cookie_auth_cli::CookieAuthArgs::parse_from([
///     "cookies", "--file", "cookies.txt", "--provider", "nextdoor-web",
/// ]);
/// assert!(!args.keep_all);
/// ```
#[derive(Parser, Debug)]
pub struct CookieAuthArgs {
    /// Provider ID to store under (e.g. nextdoor-web, gemini-web)
    #[arg(long, default_value = "nextdoor-web")]
    pub provider: String,

    /// Path to Netscape cookies export file
    #[arg(long)]
    pub file: PathBuf,

    /// Keep all valid cookies instead of filtering to auth-focused names
    #[arg(long, default_value_t = false)]
    pub keep_all: bool,
}
