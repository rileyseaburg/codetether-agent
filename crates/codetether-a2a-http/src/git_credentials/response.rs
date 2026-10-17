//! Decode credential-broker HTTP status and credential material.
use super::GitCredentialMaterial;
use anyhow::{Context, Result, anyhow};
use reqwest::StatusCode;
pub(super) async fn decode(response: reqwest::Response) -> Result<Option<GitCredentialMaterial>> {
    match response.status() {
        StatusCode::OK => Ok(Some(
            response
                .json::<GitCredentialMaterial>()
                .await
                .context("Failed to decode Git credential response")?,
        )),
        StatusCode::NOT_FOUND => Ok(None),
        status => Err(anyhow!(
            "Git credential request failed with {}: {}",
            status,
            response.text().await.unwrap_or_default()
        )),
    }
}
