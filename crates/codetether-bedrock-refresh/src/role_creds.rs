//! Exchange an SSO access token for STS role credentials.
//!
//! Calls `GetRoleCredentials` on `portal.sso.<region>.amazonaws.com`, which
//! takes the OIDC access token plus an account id + role name and returns
//! short-term AWS credentials usable to sign a Bedrock API key.

use super::exported::Exported;
use super::role_creds_types::{RoleCredsEnvelope, to_exported};
use anyhow::{Context, Result, bail};
use reqwest::Client;

/// Fetch STS role credentials for `account_id`/`role_name`.
pub(super) async fn get_role_credentials(
    http: &Client,
    region: &str,
    access_token: &str,
    account_id: &str,
    role_name: &str,
) -> Result<Exported> {
    let url = format!(
        "https://portal.sso.{region}.amazonaws.com/federation/credentials\
         ?account_id={account_id}&role_name={role_name}"
    );
    let resp = http
        .get(url)
        .header("x-amz-sso_bearer_token", access_token)
        .send()
        .await
        .context("GetRoleCredentials request failed")?;
    if !resp.status().is_success() {
        bail!(
            "GetRoleCredentials HTTP {}: {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        );
    }
    let env: RoleCredsEnvelope = resp.json().await.context("GetRoleCredentials: bad JSON")?;
    Ok(to_exported(env.role_credentials))
}
