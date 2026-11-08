//! HTTPS-only OAuth client and bounded JSON response decoding.
use anyhow::{Result, anyhow, ensure};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use std::time::Duration;

pub(crate) fn client() -> Result<Client> {
    Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .https_only(true)
        .build()
        .map_err(|_| anyhow!("Could not initialize Monarch HTTPS client"))
}

pub(crate) async fn json<T: DeserializeOwned>(response: Response) -> Result<T> {
    ensure!(
        response.status().is_success(),
        "Monarch OAuth request was rejected"
    );
    let mut response = response;
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| anyhow!("Monarch response failed"))?
    {
        ensure!(
            bytes.len() + chunk.len() <= 131_072,
            "Monarch response exceeded size limit"
        );
        bytes.extend_from_slice(&chunk);
    }
    serde_json::from_slice(&bytes).map_err(|_| anyhow!("Invalid Monarch OAuth response"))
}
