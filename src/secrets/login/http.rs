//! Auth HTTP boundary: no redirects and no credential-bearing error bodies.

use anyhow::{Result, ensure};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub(crate) fn client() -> Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30))
        .build()?)
}

pub(crate) async fn json<T: DeserializeOwned>(request: reqwest::RequestBuilder) -> Result<T> {
    let response = request
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Authentication request could not reach its endpoint"))?;
    ensure!(
        response.status().is_success(),
        "Authentication request rejected (HTTP {})",
        response.status().as_u16()
    );
    response
        .json()
        .await
        .map_err(|_| anyhow::anyhow!("Authentication response is invalid"))
}

pub(crate) async fn post<T: DeserializeOwned>(
    address: &str,
    path: &str,
    body: &Value,
) -> Result<T> {
    json(client()?.post(format!("{address}/v1/{path}")).json(body)).await
}

pub(crate) use super::auth_path::auth_path;
