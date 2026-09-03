//! Metadata-preserving Vault save for refreshed Bedrock keys.
//!
//! Reuses the SSO metadata already stored in the previous secret and only
//! updates the api_key, expiry fields, and — when the IdP rotated it — the
//! `sso_refresh_token`. Never re-reads `~/.aws/config`.

use super::exported::Exported;
use crate::secrets::{self, ProviderSecrets};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde_json::json;

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

/// Pure secret construction: prior metadata + new token/expiry/rotation.
pub(super) fn build(
    prior: &ProviderSecrets,
    token: &str,
    expires_at: DateTime<Utc>,
    exported: &Exported,
) -> ProviderSecrets {
    let mut extra = prior.extra.clone();
    extra.insert("api_key_expires_at".into(), json!(expires_at.timestamp()));
    extra.insert(
        "api_key_expires_at_rfc3339".into(),
        json!(expires_at.to_rfc3339()),
    );
    if let Some(cred) = exported.expiration {
        extra.insert(
            "credential_expires_at_rfc3339".into(),
            json!(cred.to_rfc3339()),
        );
    }
    if let Some(rotated) = &exported.rotated_refresh_token {
        extra.insert("sso_refresh_token".into(), json!(rotated));
        tracing::info!(provider = "bedrock", "persisted rotated SSO refresh token");
    }
    ProviderSecrets {
        api_key: Some(token.to_string()),
        base_url: prior.base_url.clone(),
        organization: prior.organization.clone(),
        headers: prior.headers.clone(),
        extra,
    }
}

#[cfg(test)]
#[path = "save_tests.rs"]
mod tests;
