//! `codetether auth bedrock --refresh` — silently re-mint the Bedrock key.
//!
//! Reads the SSO refresh material stored alongside the `bedrock` provider in
//! Vault, runs a browser-free OIDC `refresh_token` grant + `GetRoleCredentials`,
//! mints a fresh bearer token, and writes it back — preserving the stored SSO
//! metadata. Fails clearly when no refresh metadata was saved or the SSO
//! refresh token has expired.

use crate::provider::bedrock::token_gen;
use crate::secrets::{self, ProviderSecrets};
use anyhow::{Context, Result, bail};

use super::{
    args::BedrockAuthArgs, native_sso, refresh_emit, refresh_save, refresh_stored, validate,
    vault_expiry,
};

/// Refresh the stored Bedrock key without any browser interaction.
pub(super) async fn run(args: &BedrockAuthArgs) -> Result<()> {
    let secret: ProviderSecrets = secrets::get_provider_secrets("bedrock")
        .await
        .context("No saved 'bedrock' provider secret to refresh")?;
    let Some(sso) = refresh_stored::from_secret(&secret) else {
        bail!(
            "Stored Bedrock secret has no SSO refresh metadata. \
             Re-run `codetether auth bedrock --device-code --save` first."
        );
    };
    let exported = native_sso::refresh(native_sso::RefreshArgs {
        region: &sso.region,
        client_id: &sso.client_id,
        client_secret: &sso.client_secret,
        refresh_token: &sso.refresh_token,
        account_id: &sso.account_id,
        role_name: &sso.role_name,
    })
    .await
    .context("Silent SSO refresh failed; the refresh token may have expired")?;
    let expires = args.expires_secs.min(token_gen::DEFAULT_EXPIRES_SECS);
    let token = token_gen::generate_bearer_token(&exported.creds, &sso.region, expires);
    let token = validate::output_token(token, &sso.region, args.no_validate).await?;
    refresh_emit::print(args, &sso.region, &token, expires);
    if args.save || args.save_only {
        let expires_at = vault_expiry::effective(expires, exported.expiration);
        refresh_save::save(&secret, token.expose(), expires_at, exported.expiration).await?;
    }
    Ok(())
}
