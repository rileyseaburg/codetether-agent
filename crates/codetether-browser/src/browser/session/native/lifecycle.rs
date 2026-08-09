//! Native browser session lifecycle operations.

use crate::browser::{BrowserError, BrowserOutput, output::Ack, request::StartRequest};
use serde_json::json;

/// Start a native TetherScript browser session.
///
/// Connection parameters such as `ws_url`, `executable_path`, and `headless`
/// describe a real browser process. The native backend has none, so a request
/// carrying them is rejected instead of silently returning a session that
/// cannot execute page JavaScript.
///
/// # Errors
///
/// Returns [`BrowserError::Unsupported`] when the caller asked for a real
/// browser process or a remote DevTools endpoint.
pub(super) async fn start(
    session: &super::super::BrowserSession,
    request: StartRequest,
) -> Result<BrowserOutput, BrowserError> {
    super::start_guard::reject_real_browser_request(&request)?;
    *session.inner.native.lock().await = Some(super::NativeRuntime::new());
    Ok(ack())
}

/// Stop the native browser session.
///
/// # Errors
///
/// This currently only returns errors for API compatibility.
pub(super) async fn stop(
    session: &super::super::BrowserSession,
) -> Result<BrowserOutput, BrowserError> {
    *session.inner.native.lock().await = None;
    Ok(ack())
}

/// Return native backend health information.
///
/// # Errors
///
/// This currently only returns errors for API compatibility.
pub(super) async fn health(
    session: &super::super::BrowserSession,
) -> Result<BrowserOutput, BrowserError> {
    let native = session.inner.native.lock().await;
    let Some(runtime) = native.as_ref() else {
        return Ok(BrowserOutput::Json(json!({
            "ok": true,
            "backend": "tetherscript-native",
            "started": false
        })));
    };
    Ok(BrowserOutput::Json(json!({
        "ok": true,
        "alive": true,
        "backend": "tetherscript-native",
        "executes_page_javascript": false,
        "runtime_proof_capable": false,
        "current": runtime.current,
        "started": true,
        "tabs": runtime.pages.len()
    })))
}

/// Return a successful acknowledgement output.
pub(super) fn ack() -> BrowserOutput {
    BrowserOutput::Ack(Ack { ok: true })
}
