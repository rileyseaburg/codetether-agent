//! Auth HTTP boundary: no redirects and no credential-bearing error bodies.

use anyhow::Result;
use serde::de::DeserializeOwned;
use serde_json::Value;

/// HTTP client configured for Vault login requests.
///
/// # Errors
///
/// Returns an error when the TLS client cannot be built.
pub fn client() -> Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30))
        .build()?)
}

/// Send `request` and decode a JSON body, redacting failure reasons.
///
/// # Errors
///
/// Returns an error on transport failure, non-success status, or bad JSON.
pub async fn json<T: DeserializeOwned>(request: reqwest::RequestBuilder) -> Result<T> {
    super::guarded::send(request, &[]).await
}

/// Like [`json`], but never surfaces a reason that echoes `token`.
pub async fn json_with_token<T: DeserializeOwned>(
    request: reqwest::RequestBuilder,
    token: &str,
) -> Result<T> {
    super::guarded::send(request, &[token]).await
}

/// POST `body` to Vault; string fields (JWTs, roles) are treated as secrets.
pub async fn post<T: DeserializeOwned>(address: &str, path: &str, body: &Value) -> Result<T> {
    let secrets: Vec<&str> = body
        .as_object()
        .map(|map| map.values().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let request = client()?.post(format!("{address}/v1/{path}")).json(body);
    super::guarded::send(request, &secrets).await
}

pub use super::auth_path::auth_path;
