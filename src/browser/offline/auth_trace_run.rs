//! Auth-trace walker. Cookie jar is tetherscript's BrowserSession when the
//! feature is on; otherwise the walker is unavailable.

use anyhow::Result;

use super::auth_trace::AuthTrace;

#[cfg(feature = "tetherscript")]
pub fn walk(url: &str, max_redirects: u8) -> Result<AuthTrace> {
    use anyhow::Context;
    use reqwest::blocking::Client;
    use reqwest::redirect::Policy;
    use tetherscript::browser_session::BrowserSession;

    use super::auth_trace::{resolve, Step};
    use super::auth_trace_serde::serialize_cookie;
    use super::cookie_parse::{self, CookieRecord};

    let client = Client::builder().redirect(Policy::none()).build()?;
    let mut session = BrowserSession::new();
    let mut steps = Vec::new();
    let mut current = url.to_string();
    let mut truncated = false;
    let limit = usize::from(max_redirects);
    for hop in 0..=limit {
        // Move session "current URL" to this hop so set_cookie scopes correctly.
        session.goto_html(current.clone(), String::new());
        let cookie = session.cookie_header(&current);
        let cookie_header_sent = (!cookie.is_empty()).then(|| cookie.clone());
        let mut req = client.get(&current);
        if let Some(c) = &cookie_header_sent { req = req.header("cookie", c.clone()); }
        let resp = req.send().with_context(|| format!("GET {current}"))?;
        let status = resp.status().as_u16();
        let location = resp.headers().get("location").map(|v| String::from_utf8_lossy(v.as_bytes()).to_string());
        let mut set_cookies: Vec<CookieRecord> = Vec::new();
        for v in resp.headers().get_all("set-cookie") {
            let raw = String::from_utf8_lossy(v.as_bytes()).to_string();
            let _ = session.set_cookie(&raw);
            if let Some(rec) = cookie_parse::parse(&raw) { set_cookies.push(rec); }
        }
        steps.push(Step { method: "GET".into(), url: current.clone(), status, location: location.clone(), cookie_header_sent, set_cookies });
        match (status, location) {
            (300..=399, Some(loc)) if hop < limit => current = resolve(&current, &loc),
            (300..=399, Some(_)) => { truncated = true; break; }
            _ => break,
        }
    }
    let cookies_after = session.cookies.iter().map(serialize_cookie).collect();
    Ok(AuthTrace { final_url: current, redirect_count: steps.len().saturating_sub(1), truncated, steps, cookies_after })
}

#[cfg(not(feature = "tetherscript"))]
pub fn walk(_: &str, _: u8) -> Result<AuthTrace> {
    anyhow::bail!("auth-trace requires the `tetherscript` feature");
}
