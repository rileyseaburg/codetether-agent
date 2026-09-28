//! Device flow needs a public IdP client and an independently authorized Vault role.
//!
//! Defaults target the CodeTether deployment (Keycloak realm `spotlessbinco.com`,
//! public client `codetether-cli`, Vault JWT mount `jwt`, role `codetether-device`),
//! so `codetether vault login device` needs no flags. Each value can be
//! overridden by flag or environment variable for other deployments.

use clap::Args;

/// Public, non-secret device authorization options.
#[derive(Args, Debug, Clone)]
pub(crate) struct DeviceArgs {
    /// OIDC issuer whose discovery document advertises device authorization.
    #[arg(
        long,
        env = "CODETETHER_VAULT_DEVICE_ISSUER",
        default_value = "https://auth.quantum-forge.io/realms/spotlessbinco.com"
    )]
    pub issuer: String,
    /// Public client ID with device authorization enabled (never a client secret).
    #[arg(long, env = "CODETETHER_VAULT_DEVICE_CLIENT_ID", default_value = "codetether-cli")]
    pub client_id: String,
    /// Vault JWT auth mount configured to trust this issuer.
    #[arg(long, env = "CODETETHER_VAULT_DEVICE_MOUNT", default_value = "jwt")]
    pub mount: String,
    /// JWT-type Vault role with app-scoped policies and the correct audience.
    #[arg(long, env = "CODETETHER_VAULT_DEVICE_ROLE", default_value = "codetether-device")]
    pub role: String,
    /// Do not open a browser; show verification instructions for another device.
    #[arg(long)]
    pub no_browser: bool,
}
