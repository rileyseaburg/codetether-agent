//! Discover device endpoints from the explicitly selected OIDC issuer.

use anyhow::{Result, ensure};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Metadata {
    pub issuer: String,
    pub device_authorization_endpoint: String,
    pub token_endpoint: String,
}
pub(super) async fn load(issuer: &str) -> Result<Metadata> {
    let issuer = crate::secrets::login::normalize(issuer)?;
    let client = crate::secrets::login::http::client()?;
    let metadata: Metadata = crate::secrets::login::http::json(
        client.get(format!("{issuer}/.well-known/openid-configuration")),
    )
    .await?;
    ensure!(
        metadata.issuer.trim_end_matches('/') == issuer,
        "OIDC issuer mismatch"
    );
    let origin = reqwest::Url::parse(&issuer)?.origin();
    for endpoint in [
        &metadata.device_authorization_endpoint,
        &metadata.token_endpoint,
    ] {
        let endpoint = crate::secrets::login::normalize(endpoint)?;
        ensure!(
            reqwest::Url::parse(&endpoint)?.origin() == origin,
            "Device endpoints must belong to the selected issuer origin"
        );
    }
    Ok(metadata)
}
