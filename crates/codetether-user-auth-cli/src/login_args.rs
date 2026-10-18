//! LoginAuthArgs command schema.
use clap::Parser;

/// Account command options with optional interactive email input.
///
/// # Examples
/// ```
/// let args = codetether_user_auth_cli::LoginAuthArgs { server: "https://example.invalid".into(), email: None };
/// assert!(args.email.is_none());
/// ```
#[derive(Parser, Debug)]
pub struct LoginAuthArgs {
    /// CodeTether server URL (e.g., https://api.codetether.io)
    #[arg(short, long, env = "CODETETHER_SERVER")]
    pub server: String,

    /// Email address
    #[arg(short, long)]
    pub email: Option<String>,
}
