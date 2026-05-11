//! Replay a captured HTTP response through a deterministic tetherscript BrowserSession.

use anyhow::Result;
use std::path::Path;

use super::record::Capture;

#[derive(Debug, serde::Serialize)]
pub struct ReplayReport {
    pub url: String,
    pub status: u16,
    pub body_bytes: usize,
    pub eval_result: String,
}

pub fn run(capture: &Path) -> Result<String> {
    let text = std::fs::read_to_string(capture)?;
    let cap: Capture = serde_json::from_str(&text)?;
    let report = replay_capture(&cap)?;
    Ok(serde_json::to_string_pretty(&report)?)
}

#[cfg(feature = "tetherscript")]
fn replay_capture(cap: &Capture) -> Result<ReplayReport> {
    use tetherscript::browser_session::BrowserSession;
    let mut session = BrowserSession::new();
    session.goto_html(cap.url.clone(), cap.body.clone());
    let eval = session
        .eval_js("document.title")
        .map(|v| format!("{:?}", v))
        .unwrap_or_else(|e| format!("eval-error: {e}"));
    Ok(ReplayReport { url: cap.url.clone(), status: cap.status, body_bytes: cap.body.len(), eval_result: eval })
}

#[cfg(not(feature = "tetherscript"))]
fn replay_capture(_: &Capture) -> Result<ReplayReport> {
    anyhow::bail!("replay requires the `tetherscript` feature");
}
