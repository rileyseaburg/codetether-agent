//! Parsing for model-independent Vault configuration and login commands.

use clap::{Args, Subcommand};

/// Vault management arguments; no subcommand displays status.
///
/// # Examples
/// ```
/// use clap::Parser;
/// let cli = codetether_agent::cli::Cli::try_parse_from(["codetether", "vault", "status"]);
/// assert!(cli.is_ok());
/// ```
#[derive(Args, Debug, Clone)]
pub struct VaultArgs {
    #[command(subcommand)]
    pub(crate) action: Option<Action>,
}
/// Operations deliberately independent of provider credentials.
#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Action {
    /// Show the effective address and credential source, never the token.
    Status,
    /// Set the active Vault URL, discarding credentials for the previous URL.
    Url { address: String },
    /// Authenticate and save a validated, non-administrator Vault credential.
    Login {
        #[command(subcommand)]
        method: Login,
    },
    /// Remove the saved local credential (does not revoke an external token).
    Logout,
}
/// Explicit login mechanisms; no token is accepted as a command-line argument.
#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Login {
    /// Hidden token prompt, or read a token from stdin for automation.
    Token {
        #[arg(long)]
        stdin: bool,
    },
    /// Browser OIDC through Vault's configured auth mount (requires Vault CLI).
    Oidc {
        #[arg(long, default_value = "oidc")]
        mount: String,
        #[arg(long, default_value = "codetether")]
        role: String,
        #[arg(long)]
        no_browser: bool,
    },
    /// IdP device authorization followed by exchange through a Vault JWT role.
    Device(super::device_args::DeviceArgs),
}
