//! Native network diagnostics.

use crate::browser::{BrowserError, BrowserOutput, request::DiagnoseRequest};
use serde_json::json;

/// Return network and backend diagnostic information.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started.
pub(in crate::browser::session::native) async fn diagnose(
    session: &super::super::super::super::BrowserSession,
    _request: DiagnoseRequest,
) -> Result<BrowserOutput, BrowserError> {
    let native = session.inner.native.lock().await;
    let page = native
        .as_ref()
        .ok_or(BrowserError::SessionNotStarted)?
        .current()?;
    Ok(BrowserOutput::Json(json!({
        "backend": "tetherscript-native",
        "url": page.session.url.clone(),
        "console_events": page.session.console.len(),
        "network_events": page.session.network.len(),
        "page_errors": []
    })))
}
