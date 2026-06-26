//! Native AWS SSO device-code flow (no `aws` CLI, no `~/.aws/config`).
//!
//! Bootstraps a Bedrock login from only a start URL + region + account/role:
//! registers an OIDC client, starts device authorization, prints/opens the
//! verification URL, polls for the token, then exchanges it for STS role
//! credentials. Produces the same [`Exported`] the CLI-based path returns.

mod oidc_device_auth;
mod oidc_poll;
mod oidc_refresh;
mod oidc_register;
mod oidc_types;
mod refresh_flow;
mod role_creds;
#[cfg(test)]
mod role_creds_tests;
mod role_creds_types;

pub(super) use refresh_flow::{RefreshArgs, refresh};

use anyhow::Result;
use reqwest::Client;

use super::exported::Exported;

/// Inputs required for a profile-free device-code login.
pub(super) struct NativeSsoArgs<'a> {
    pub start_url: &'a str,
    pub region: &'a str,
    pub account_id: &'a str,
    pub role_name: &'a str,
}

/// Run the full native device-code flow and return role credentials.
pub(super) async fn login(args: NativeSsoArgs<'_>) -> Result<Exported> {
    let http = Client::builder()
        .user_agent("codetether-bedrock-auth")
        .build()?;
    let client = oidc_register::register_client(&http, args.region).await?;
    let auth =
        oidc_register::start_device_authorization(&http, args.region, &client, args.start_url)
            .await?;
    prompt_user(&auth);
    let token = oidc_poll::poll_for_token(&http, args.region, &client, &auth).await?;
    role_creds::get_role_credentials(
        &http,
        args.region,
        &token.access_token,
        args.account_id,
        args.role_name,
    )
    .await
}

/// Print the verification URL/code and attempt to open the browser.
fn prompt_user(auth: &oidc_types::DeviceAuthorization) {
    let url = auth
        .verification_uri_complete
        .as_deref()
        .unwrap_or(&auth.verification_uri);
    eprintln!("To authorize, open: {url}");
    eprintln!("Then enter the code: {}", auth.user_code);
    let _ = open::that_detached(url);
}
