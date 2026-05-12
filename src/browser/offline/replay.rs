//! Replay a captured HTTP response through a deterministic tetherscript BrowserSession.

use anyhow::Result;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
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
    let eval = session
        .eval_js("document.title")
        .map(|v| format!("{:?}", v))
        .unwrap_or_else(|e| format!("eval-error: {e}"));
    Ok(ReplayReport {
        url: cap.url.clone(),
        status: cap.status,
        body_bytes: body.len(),
        content_type: cap.content_type.clone(),
        eval_result: eval,
    })
}

#[cfg(not(feature = "tetherscript"))]
fn replay_capture(_: &Capture, _: &[u8]) -> Result<ReplayReport> {
    anyhow::bail!("replay requires the `tetherscript` feature");
}
