//! Record an auth flow: walk redirect chain, capture Set-Cookie per hop, emit JSON trace.

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::redirect::Policy;

use super::cookie_parse::{self, CookieRecord};

#[derive(Debug, serde::Serialize)]
pub struct AuthTrace {
    pub final_url: String,
    pub redirect_count: usize,
    pub steps: Vec<Step>,
    pub cookies_after: Vec<CookieRecord>,
}

#[derive(Debug, serde::Serialize)]
pub struct Step {
    pub method: String,
    pub url: String,
    pub status: u16,
    pub location: Option<String>,
    pub set_cookies: Vec<CookieRecord>,
}

pub fn run(url: &str, max_redirects: u8) -> Result<String> {
    let client = Client::builder().redirect(Policy::none()).build()?;
    let mut steps = Vec::new();
    let mut cookies_after: Vec<CookieRecord> = Vec::new();
    let mut current = url.to_string();
    for _ in 0..=max_redirects {
        let resp = client.get(&current).send().with_context(|| format!("GET {current}"))?;
        let status = resp.status().as_u16();
        let location = resp.headers().get("location").and_then(|v| v.to_str().ok()).map(String::from);
        let set_cookies: Vec<CookieRecord> = resp
            .headers()
            .get_all("set-cookie")
            .iter()
            .filter_map(|v| v.to_str().ok().and_then(cookie_parse::parse))
            .collect();
        cookies_after.extend(set_cookies.iter().cloned());
        steps.push(Step { method: "GET".into(), url: current.clone(), status, location: location.clone(), set_cookies });
        match (status, location) {
            (300..=399, Some(loc)) => current = resolve(&current, &loc),
            _ => break,
        }
    }
    let trace = AuthTrace { final_url: current, redirect_count: steps.len().saturating_sub(1), steps, cookies_after };
    Ok(serde_json::to_string_pretty(&trace)?)
}

fn resolve(base: &str, location: &str) -> String {
    reqwest::Url::parse(base).and_then(|b| b.join(location)).map(|u| u.to_string()).unwrap_or_else(|_| location.to_string())
}
