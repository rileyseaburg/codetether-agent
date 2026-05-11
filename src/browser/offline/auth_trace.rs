//! Walk an HTTP redirect chain, carry cookies hop-to-hop via tetherscript
//! BrowserSession's jar, emit JSON trace.

use anyhow::Result;

use super::auth_trace_run;
use super::cookie_parse::CookieRecord;

#[derive(Debug, serde::Serialize)]
pub struct AuthTrace {
    pub final_url: String,
    pub redirect_count: usize,
    pub truncated: bool,
    pub steps: Vec<Step>,
    pub cookies_after: Vec<CookieRecord>,
}

#[derive(Debug, serde::Serialize)]
pub struct Step {
    pub method: String,
    pub url: String,
    pub status: u16,
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_header_sent: Option<String>,
    pub set_cookies: Vec<CookieRecord>,
}

pub fn run(url: &str, max_redirects: u8) -> Result<String> {
    let trace = auth_trace_run::walk(url, max_redirects)?;
    Ok(serde_json::to_string_pretty(&trace)?)
}

pub(crate) fn resolve(base: &str, location: &str) -> String {
    reqwest::Url::parse(base)
        .and_then(|b| b.join(location))
        .map(|u| u.to_string())
        .unwrap_or_else(|_| location.to_string())
}
