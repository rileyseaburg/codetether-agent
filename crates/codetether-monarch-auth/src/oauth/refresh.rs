//! Refresh token exchange; production always uses the pinned token endpoint.
use super::{credentials::Credentials, token::credentials};
use crate::discovery::{self, RESOURCE, TOKEN};
use anyhow::{Result, anyhow};
use reqwest::Client;

pub(crate) async fn refresh(client: &Client, old: &Credentials) -> Result<Credentials> {
    refresh_at(client, old, TOKEN).await
}

pub(super) async fn refresh_at(
    client: &Client,
    old: &Credentials,
    endpoint: &str,
) -> Result<Credentials> {
    let refresh_token = old
        .refresh_token
        .as_deref()
        .ok_or_else(|| anyhow!("Monarch session expired; run codetether auth monarch login"))?;
    let response = client
        .post(endpoint)
        .form(&[
            ("grant_type", "refresh_token"),
            ("client_id", &old.client_id),
            ("refresh_token", refresh_token),
            ("resource", RESOURCE),
        ])
        .send()
        .await
        .map_err(|_| {
            anyhow!("Monarch token refresh failed; retry or run codetether auth monarch login")
        })?;
    credentials(
        discovery::json(response).await?,
        &old.client_id,
        &old.scope,
        Some(refresh_token),
    )
}
