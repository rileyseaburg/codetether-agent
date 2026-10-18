//! RegisterAuthArgs command schema.
use clap::Parser;

/// Account command options with optional interactive email input.
///
/// # Examples
/// ```
/// let args = codetether_user_auth_cli::RegisterAuthArgs { server: "https://example.invalid".into(), email: None, first_name: None, last_name: None, referral_source: None };
/// assert!(args.email.is_none());
/// ```
#[derive(Parser, Debug)]
pub struct RegisterAuthArgs {
    /// CodeTether server URL (e.g., https://api.codetether.run)
    #[arg(short, long, env = "CODETETHER_SERVER")]
    pub server: String,

    /// Email address
    #[arg(short, long)]
    pub email: Option<String>,

    /// First name (optional)
    #[arg(long)]
    pub first_name: Option<String>,

    /// Last name (optional)
    #[arg(long)]
    pub last_name: Option<String>,

    /// Referral source (optional)
    #[arg(long)]
    pub referral_source: Option<String>,
}
