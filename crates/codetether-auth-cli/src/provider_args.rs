//! Authentication provider_args responsibilities.
use clap::Parser;

/// Provider-specific authentication options.
///
/// # Examples
/// ```
/// let args = codetether_auth_cli::CopilotAuthArgs { enterprise_url: None, client_id: None };
/// ```
#[derive(Parser, Debug)]
pub struct CopilotAuthArgs {
    /// GitHub Enterprise URL or domain (e.g. company.ghe.com)
    #[arg(long)]
    pub enterprise_url: Option<String>,

    /// GitHub OAuth app client ID for Copilot device flow
    #[arg(long, env = "CODETETHER_COPILOT_OAUTH_CLIENT_ID")]
    pub client_id: Option<String>,
}

/// Provider-specific authentication options.
///
/// # Examples
/// ```
/// let args = codetether_auth_cli::CodexAuthArgs { device_code: true };
/// ```
#[derive(Parser, Debug)]
pub struct CodexAuthArgs {
    /// Use ChatGPT device-code authentication flow (recommended for remote SSH sessions)
    #[arg(long, default_value_t = false)]
    pub device_code: bool,
}
