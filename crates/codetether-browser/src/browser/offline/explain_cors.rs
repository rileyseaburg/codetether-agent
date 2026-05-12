//! Send a CORS preflight (OPTIONS) and classify whether the cross-origin call would be allowed.

use anyhow::Result;
use reqwest::blocking::Client;
use std::collections::BTreeMap;

use super::explain_cors_analyse::header_reasons;

#[derive(Debug, serde::Serialize)]
pub struct CorsExplanation {
    pub allowed: bool,
    pub target: String,
    pub origin: String,
    pub method: String,
    pub preflight_status: u16,
    pub reasons: Vec<String>,
    pub response_headers: BTreeMap<String, String>,
}

pub fn run(url: &str, origin: &str, method: &str) -> Result<String> {
    let client = Client::builder().build()?;
    let resp = client
        .request(reqwest::Method::OPTIONS, url)
        .header("origin", origin)
        .header("access-control-request-method", method)
        .send()?;
    let preflight_status = resp.status().as_u16();
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
    let mut reasons = Vec::new();
    if !(200..300).contains(&preflight_status) {
        reasons.push(format!(
            "preflight returned non-2xx status {preflight_status}"
        ));
    }
    reasons.extend(header_reasons(&headers, origin, method));
    Ok(serde_json::to_string_pretty(&CorsExplanation {
        allowed: reasons.is_empty(),
        target: url.into(),
        origin: origin.into(),
        method: method.into(),
        preflight_status,
        reasons,
        response_headers: headers,
    })?)
}
