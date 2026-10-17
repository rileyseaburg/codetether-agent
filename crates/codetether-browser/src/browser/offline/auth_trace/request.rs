//! Execute one redirect hop and update the shared cookie jar.
use super::Step;
use crate::browser::offline::cookie_parse::{self, CookieRecord};
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use tetherscript::browser_session::BrowserSession;
pub(super) fn fetch(client: &Client, session: &mut BrowserSession, current: &str) -> Result<Step> {
    // Move session "current URL" to this hop so set_cookie scopes correctly.
    session.goto_html(current.to_string(), String::new());
    let cookie = session.cookie_header(&current);
    let cookie_header_sent = (!cookie.is_empty()).then(|| cookie.clone());
    let mut req = client.get(current);
    if let Some(c) = &cookie_header_sent {
        req = req.header("cookie", c.clone());
    }
    let resp = req.send().with_context(|| format!("GET {current}"))?;
    let status = resp.status().as_u16();
    let location = resp
        .headers()
        .get("location")
        .map(|v| String::from_utf8_lossy(v.as_bytes()).to_string());
    let mut set_cookies: Vec<CookieRecord> = Vec::new();
    for v in resp.headers().get_all("set-cookie") {
        let raw = String::from_utf8_lossy(v.as_bytes()).to_string();
        let _ = session.set_cookie(&raw);
        if let Some(rec) = cookie_parse::parse(&raw) {
            set_cookies.push(rec);
        }
    }
    Ok(Step {
        method: "GET".into(),
        url: current.to_string(),
        status,
        location: location.clone(),
        cookie_header_sent,
        set_cookies,
    })
}
