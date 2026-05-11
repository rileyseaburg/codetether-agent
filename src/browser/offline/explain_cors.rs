//! Send a CORS preflight (OPTIONS) and classify whether the cross-origin call would be allowed.

use anyhow::Result;
use reqwest::blocking::Client;
use std::collections::BTreeMap;

#[derive(Debug, serde::Serialize)]
pub struct CorsExplanation {
    pub allowed: bool,
    pub target: String,
    pub origin: String,
    pub method: String,
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
    let headers: BTreeMap<String, String> = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().to_ascii_lowercase(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let reasons = analyse(&headers, origin, method);
    let explanation = CorsExplanation {
        allowed: reasons.is_empty(),
        target: url.into(),
        origin: origin.into(),
        method: method.into(),
        reasons,
        response_headers: headers,
    };
    Ok(serde_json::to_string_pretty(&explanation)?)
}

fn analyse(headers: &BTreeMap<String, String>, origin: &str, method: &str) -> Vec<String> {
    let mut reasons = Vec::new();
    let allow_origin = headers.get("access-control-allow-origin").map(String::as_str).unwrap_or("");
    if allow_origin != "*" && allow_origin != origin {
        reasons.push(format!("Access-Control-Allow-Origin = '{allow_origin}', does not match '{origin}'"));
    }
    let allow_methods = headers.get("access-control-allow-methods").map(String::as_str).unwrap_or("");
    if !allow_methods.split(',').any(|m| m.trim().eq_ignore_ascii_case(method)) {
        reasons.push(format!("method {method} not in Access-Control-Allow-Methods = '{allow_methods}'"));
    }
    reasons
}
