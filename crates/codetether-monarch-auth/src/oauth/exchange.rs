//! Exchange a loopback authorization code using the original PKCE verifier.
use super::{credentials::Credentials, token::credentials};
use crate::discovery::{self, RESOURCE, TOKEN};
use anyhow::{Result, anyhow};
use reqwest::Client;

pub(super) async fn exchange(
    client: &Client,
    client_id: &str,
    redirect: &str,
    code: &str,
    verifier: &str,
    scope: &str,
) -> Result<Credentials> {
    let response = client
        .post(TOKEN)
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", client_id),
            ("code", code),
            ("code_verifier", verifier),
            ("redirect_uri", redirect),
            ("resource", RESOURCE),
        ])
        .send()
        .await
        .map_err(|_| anyhow!("Monarch code exchange failed"))?;
    credentials(discovery::json(response).await?, client_id, scope, None)
}
