//! Credential resolution for `codetether auth bedrock`.
//!
//! Tries env vars / `~/.aws/credentials` first, then
//! `aws configure export-credentials`, which resolves SSO/IdP sessions,
//! assumed roles, and credential processes. Explicit browser/device-code
//! modes force a fresh login before exporting credentials.

use super::aws_cli;
use super::login::LoginMode;
use crate::provider::bedrock::AwsCredentials;
use anyhow::Result;

/// Resolve AWS credentials, logging in when requested or required.
pub(super) async fn resolve(profile: Option<&str>, mode: LoginMode) -> Result<AwsCredentials> {
    if profile.is_none() && matches!(mode, LoginMode::Browser | LoginMode::DeviceCode) {
        anyhow::bail!(
            "Forced SSO login requires an SSO profile. Use --sso <start-url> or --profile <name>."
        );
    }
    if profile.is_none()
        && let Some(creds) = AwsCredentials::from_environment()
    {
        eprintln!("Using AWS credentials from environment/shared credentials.");
        return Ok(creds);
    }
    if let Some(p) = profile
        && matches!(mode, LoginMode::Browser | LoginMode::DeviceCode)
    {
        super::login::run(p, mode).await?;
    }
    match aws_cli::export_credentials(profile).await {
        Ok(creds) => Ok(creds),
        Err(e) => retry_after_login(profile, mode, e).await,
    }
}

/// Retry credential export after an auto-login attempt.
async fn retry_after_login(
    profile: Option<&str>,
    mode: LoginMode,
    e: anyhow::Error,
) -> Result<AwsCredentials> {
    match profile {
        Some(p) if mode != LoginMode::Off => {
            tracing::info!(profile = p, "SSO export failed; attempting login: {e}");
            super::login::run(p, mode).await?;
            aws_cli::export_credentials(profile).await
        }
        _ => Err(e),
    }
}
