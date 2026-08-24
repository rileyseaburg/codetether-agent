//! Network-gated loading of remote image bytes.

use super::ImageTool;
use anyhow::{Result, bail};
use serde_json::Value;

pub(super) async fn load(url: &str, args: &Value) -> Result<(String, String, usize, String)> {
    crate::tool::network_access::require("image", args)?;
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        bail!("Failed to fetch image from URL: HTTP {}", response.status());
    }
    let mime = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
        .unwrap_or_else(|| ImageTool::detect_mime_type(url).to_string());
    let bytes = response.bytes().await?;
    let encoded = ImageTool::encode_as_data_url(&bytes, &mime);
    Ok((encoded, mime, bytes.len(), url.to_string()))
}