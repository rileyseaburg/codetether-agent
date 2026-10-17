//! Metadata-preserving Vault save for refreshed Bedrock keys.
//!
//! Reuses the SSO metadata already stored in the previous secret and only
//! updates the api_key, expiry fields, and — when the IdP rotated it — the
//! `sso_refresh_token`. Never re-reads `~/.aws/config`.

use super::exported::Exported;
use crate::save_build::build;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use codetether_secrets::{self as secrets, ProviderSecrets};

/// Re-save the `bedrock` secret with a new token, preserving SSO metadata.
pub(crate) async fn save(
    prior: &ProviderSecrets,
    token: &str,
    expires_at: DateTime<Utc>,
    exported: &Exported,
) -> Result<()> {
    let secret = build(prior, token, expires_at, exported);
    secrets::set_provider_secrets("bedrock", &secret)
        .await
        .context("Failed to re-save refreshed Bedrock token in Vault")
}
