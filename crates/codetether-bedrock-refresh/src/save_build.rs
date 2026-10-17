//! Preserve provider metadata while replacing refreshed token fields.
use crate::exported::Exported;
use chrono::{DateTime, Utc};
use codetether_secrets::ProviderSecrets;
use serde_json::json;

/// Pure secret construction: prior metadata + new token/expiry/rotation.
pub(crate) fn build(
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
