//! Capture an HTTP GET (status + headers + body) to JSON. Body is base64
//! so binary payloads survive a round-trip through replay.

use anyhow::Result;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use reqwest::blocking::Client;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Capture {
    pub url: String,
    pub status: u16,
    pub headers: BTreeMap<String, String>,
    pub content_type: Option<String>,
    /// Response body, base64-encoded so binary payloads survive serialization.
    pub body_base64: String,
}

pub fn run(url: &str, out: &Path) -> Result<String> {
    let resp = Client::builder().build()?.get(url).send()?;
    let status = resp.status().as_u16();
    let headers: BTreeMap<String, String> = resp
        .headers()
        .iter()
        .map(|(k, v)| {
            (
                k.as_str().to_ascii_lowercase(),
                String::from_utf8_lossy(v.as_bytes()).to_string(),
            )
        })
        .collect();
    let content_type = headers.get("content-type").cloned();
    let bytes = resp.bytes()?;
    let capture = Capture {
        url: url.into(),
        status,
        headers,
        content_type,
        body_base64: STANDARD.encode(&bytes),
    };
    std::fs::write(out, serde_json::to_string_pretty(&capture)?)?;
    Ok(serde_json::to_string_pretty(&serde_json::json!({
        "saved": out.display().to_string(),
        "status": capture.status,
        "bytes": bytes.len(),
        "content_type": capture.content_type,
    }))?)
}
