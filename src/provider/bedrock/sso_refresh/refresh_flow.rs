//! Silent SSO refresh: stored refresh token → fresh role credentials.
//!
//! Uses the OIDC `refresh_token` grant (no browser) followed by
//! `GetRoleCredentials` to produce [`Exported`] credentials. Valid only while
//! the SSO refresh token is still alive.

use super::exported::Exported;
use super::oidc_refresh::{RefreshInputs, refresh_access_token};
use super::role_creds::get_role_credentials;
use anyhow::Result;
use reqwest::Client;

/// All material needed to silently refresh role credentials.
pub(crate) struct RefreshArgs<'a> {
    pub region: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub refresh_token: &'a str,
    pub account_id: &'a str,
    pub role_name: &'a str,
}

/// Run a non-interactive refresh and return fresh STS role credentials.
pub(crate) async fn refresh(args: RefreshArgs<'_>) -> Result<Exported> {
    let http = Client::builder()
        .user_agent("codetether-bedrock-auth")
        .build()?;
    let token = refresh_access_token(
        &http,
        RefreshInputs {
            region: args.region,
            client_id: args.client_id,
            client_secret: args.client_secret,
            refresh_token: args.refresh_token,
        },
    )
    .await?;
    get_role_credentials(
        &http,
        args.region,
        &token.access_token,
        args.account_id,
        args.role_name,
    )
    .await
}
