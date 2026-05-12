//! Replay captured HTTP responses through tetherscript.

use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use std::path::Path;

use super::record::Capture;

#[derive(Debug, serde::Serialize)]
pub struct ReplayReport {
    pub url: String,
    pub status: u16,
    pub body_bytes: usize,
    pub content_type: Option<String>,
    pub eval_result: String,
}

pub fn run(capture: &Path) -> Result<String> {
    let text = std::fs::read_to_string(capture)?;
    let cap: Capture = serde_json::from_str(&text)?;
    let body = STANDARD.decode(cap.body_base64.as_bytes())?;
    let report = replay_capture(&cap, &body)?;
    Ok(serde_json::to_string_pretty(&report)?)
}

#[cfg(feature = "tetherscript")]
fn replay_capture(cap: &Capture, body: &[u8]) -> Result<ReplayReport> {
    use tetherscript::browser_session::BrowserSession;
    let html = String::from_utf8_lossy(body).to_string();
    let mut session = BrowserSession::new();
    session.goto_html(cap.url.clone(), html);
    Ok(report(cap, body, eval_title(&mut session)))
}

#[cfg(feature = "tetherscript")]
fn eval_title(session: &mut tetherscript::browser_session::BrowserSession) -> String {
    session
        .eval_js("document.title")
        .map(|value| format!("{value:?}"))
        .unwrap_or_else(|error| format!("eval-error: {error}"))
}

#[cfg(not(feature = "tetherscript"))]
fn replay_capture(_: &Capture, _: &[u8]) -> Result<ReplayReport> {
    anyhow::bail!("replay requires the `tetherscript` feature");
}

fn report(cap: &Capture, body: &[u8], eval_result: String) -> ReplayReport {
    ReplayReport {
        url: cap.url.clone(),
        status: cap.status,
        body_bytes: body.len(),
        content_type: cap.content_type.clone(),
        eval_result,
    }
}
