//! Send one auth request; on rejection report only a leak-free reason.

use anyhow::Result;
use serde::de::DeserializeOwned;

/// Send `request` and decode JSON; `secrets` are values Vault must not echo.
pub(super) async fn send<T: DeserializeOwned>(
    request: reqwest::RequestBuilder,
    secrets: &[&str],
) -> Result<T> {
    let response = request
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Authentication request could not reach its endpoint"))?;
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "{}",
            super::rejection::message(status.as_u16(), &body, secrets)
        );
    }
    response
        .json()
        .await
        .map_err(|_| anyhow::anyhow!("Authentication response is invalid"))
}
