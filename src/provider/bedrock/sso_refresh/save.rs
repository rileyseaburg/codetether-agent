//! Metadata-preserving Vault save for refreshed Bedrock keys.
//!
//! Reuses the SSO metadata already stored in the previous secret and only
//! updates the api_key and expiry fields. Never re-reads `~/.aws/config`.

use crate::secrets::{self, ProviderSecrets};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde_json::json;

/// Re-save the `bedrock` secret with a new token, preserving SSO metadata.
pub(crate) async fn save(
    prior: &ProviderSecrets,
    token: &str,
    expires_at: DateTime<Utc>,
    cred_expiration: Option<DateTime<Utc>>,
) -> Result<()> {
    let mut extra = prior.extra.clone();
    extra.insert("api_key_expires_at".into(), json!(expires_at.timestamp()));
    extra.insert(
        "api_key_expires_at_rfc3339".into(),
        json!(expires_at.to_rfc3339()),
    );
    if let Some(cred) = cred_expiration {
        extra.insert(
            "credential_expires_at_rfc3339".into(),
            json!(cred.to_rfc3339()),
        );
    }
    let secret = ProviderSecrets {
        api_key: Some(token.to_string()),
        base_url: prior.base_url.clone(),
        organization: prior.organization.clone(),
        headers: prior.headers.clone(),
        extra,
    };
    secrets::set_provider_secrets("bedrock", &secret)
        .await
        .context("Failed to re-save refreshed Bedrock token in Vault")
}
