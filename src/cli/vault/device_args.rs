//! Device flow needs a public IdP client and an independently authorized Vault role.

use clap::Args;

/// Public, non-secret device authorization options.
#[derive(Args, Debug, Clone)]
pub(crate) struct DeviceArgs {
    /// OIDC issuer whose discovery document advertises device authorization.
    #[arg(long)]
    pub issuer: String,
    /// Public client ID with device authorization enabled (never a client secret).
    #[arg(long)]
    pub client_id: String,
    /// Vault JWT/OIDC auth mount configured to trust this issuer.
    #[arg(long, default_value = "oidc")]
    pub mount: String,
    /// JWT-type Vault role with app-scoped policies and the correct audience.
    #[arg(long, default_value = "codetether-device")]
    pub role: String,
    /// Do not open a browser; show verification instructions for another device.
    #[arg(long)]
    pub no_browser: bool,
}
