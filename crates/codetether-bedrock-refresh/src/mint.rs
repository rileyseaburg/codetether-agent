//! Refresh role credentials, mint a token, and optionally persist metadata.
use crate::auto::Refreshed;
use crate::{refresh_flow, save, stored, vault_expiry};
use anyhow::{Context, Result, bail};
use codetether_bedrock_auth::token_gen;
use codetether_secrets::ProviderSecrets;

pub(crate) async fn refresh_secret(
    secret: &ProviderSecrets,
    save_to_vault: bool,
) -> Result<Refreshed> {
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
        save::save(secret, &token, expires_at, &exported).await?;
    }
    Ok(Refreshed {
        token,
        region: sso.region,
    })
}
