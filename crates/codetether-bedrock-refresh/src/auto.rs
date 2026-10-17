//! Proactive, browser-free refresh of a stored Bedrock API key.
//!
//! Shared by startup auto-assessment ([`ensure_fresh`]) and the CLI
//! `--refresh` command ([`refresh_now`]): both run the OIDC `refresh_token`
//! grant + `GetRoleCredentials`, re-mint the bearer token, and re-save it
//! while preserving the stored SSO metadata.

use super::staleness;
use crate::mint::refresh_secret;
use anyhow::{Context, Result};
use codetether_secrets::{self as secrets};

/// Outcome of a silent refresh: the freshly minted bearer token + region.
///
/// # Examples
/// ```
/// let refreshed = codetether_bedrock_refresh::Refreshed { token: "token".into(), region: "us-east-1".into() };
/// assert_eq!(refreshed.region, "us-east-1");
/// ```
pub struct Refreshed {
    /// Freshly generated Bedrock bearer token.
    pub token: String,
    /// AWS region used to obtain and sign credentials.
    pub region: String,
}

/// If the stored `bedrock` key is stale and SSO refresh metadata exists,
/// silently re-mint and re-save it. Returns the new token when refreshed,
/// `None` when no refresh was needed or possible.
///
/// # Arguments
/// Reads the stored Bedrock secret through the shared secrets service.
///
/// # Returns
/// A refreshed token, or none if no refresh occurred. Failures are logged.
///
/// # Examples
/// ```no_run
/// # async fn example() {
/// let token = codetether_bedrock_refresh::ensure_fresh().await;
/// # }
/// ```
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
///
/// # Arguments
/// * `save_to_vault` - Whether to persist the new token and refreshed metadata.
///
/// # Returns
/// The new token and signing region.
///
/// # Errors
/// Returns an error for missing stored SSO metadata, failed credential exchange,
/// or a failed requested secret-store write.
///
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let refreshed = codetether_bedrock_refresh::refresh_now(false).await?;
/// assert!(!refreshed.token.is_empty());
/// # Ok(())
/// # }
/// ```
pub async fn refresh_now(save_to_vault: bool) -> Result<Refreshed> {
    let secret = secrets::get_provider_secrets("bedrock")
        .await
        .context("No saved 'bedrock' provider secret to refresh")?;
    refresh_secret(&secret, save_to_vault).await
}
