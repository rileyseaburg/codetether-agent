//! Capture an HTTP GET response (status + headers + body) to a JSON file for later replay.

use anyhow::Result;
use reqwest::blocking::Client;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Capture {
    pub url: String,
    pub status: u16,
    pub headers: BTreeMap<String, String>,
    pub body: String,
}

pub fn run(url: &str, out: &Path) -> Result<String> {
    let resp = Client::builder().build()?.get(url).send()?;
    let status = resp.status().as_u16();
    let headers: BTreeMap<String, String> = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().to_ascii_lowercase(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let body = resp.text()?;
    let capture = Capture { url: url.into(), status, headers, body };
    std::fs::write(out, serde_json::to_string_pretty(&capture)?)?;
    Ok(serde_json::to_string_pretty(&serde_json::json!({
        "saved": out.display().to_string(),
        "status": capture.status,
        "bytes": capture.body.len(),
    }))?)
}
