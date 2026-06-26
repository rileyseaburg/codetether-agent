//! Proactive, browser-free refresh of a stored Bedrock API key.
//!
//! Shared by startup auto-assessment ([`ensure_fresh`]) and the CLI
//! `--refresh` command ([`refresh_now`]): both run the OIDC `refresh_token`
//! grant + `GetRoleCredentials`, re-mint the bearer token, and re-save it
//! while preserving the stored SSO metadata.

use super::{refresh_flow, save, staleness, stored, vault_expiry};
use crate::provider::bedrock::token_gen;
use crate::secrets::{self, ProviderSecrets};
use anyhow::{Context, Result, bail};

/// Outcome of a silent refresh: the freshly minted bearer token + region.
pub(crate) struct Refreshed {
    pub token: String,
    pub region: String,
}

/// If the stored `bedrock` key is stale and SSO refresh metadata exists,
/// silently re-mint and re-save it. Returns the new token when refreshed,
/// `None` when no refresh was needed or possible.
pub async fn ensure_fresh() -> Option<String> {
    let secret = secrets::get_provider_secrets("bedrock").await?;
    if !staleness::is_stale(&secret) {
        return None;
    }
    match refresh_secret(&secret, true).await {
        Ok(r) => {
            tracing::info!(provider = "bedrock", "silently refreshed stale Bedrock key");
            Some(r.token)
        }
        Err(e) => {
            tracing::warn!(provider = "bedrock", error = %e, "auto-refresh skipped");
            None
        }
    }
}

/// Force a refresh of the stored `bedrock` secret (CLI `--refresh`).
pub(crate) async fn refresh_now(save_to_vault: bool) -> Result<Refreshed> {
    let secret = secrets::get_provider_secrets("bedrock")
        .await
        .context("No saved 'bedrock' provider secret to refresh")?;
    refresh_secret(&secret, save_to_vault).await
}

async fn refresh_secret(secret: &ProviderSecrets, save_to_vault: bool) -> Result<Refreshed> {
    let Some(sso) = stored::from_secret(secret) else {
        bail!(
            "Stored Bedrock secret has no SSO refresh metadata. \
             Re-run `codetether auth bedrock --device-code --save` first."
        );
    };
    let exported = refresh_flow::refresh(refresh_flow::RefreshArgs {
        region: &sso.region,
        client_id: &sso.client_id,
        client_secret: &sso.client_secret,
        refresh_token: &sso.refresh_token,
        account_id: &sso.account_id,
        role_name: &sso.role_name,
    })
    .await
    .context("Silent SSO refresh failed; the refresh token may have expired")?;
    let expires = token_gen::DEFAULT_EXPIRES_SECS;
    let token = token_gen::generate_bearer_token(&exported.creds, &sso.region, expires);
    if save_to_vault {
        let expires_at = vault_expiry::effective(expires, exported.expiration);
        save::save(secret, &token, expires_at, exported.expiration).await?;
    }
    Ok(Refreshed {
        token,
        region: sso.region,
    })
}
