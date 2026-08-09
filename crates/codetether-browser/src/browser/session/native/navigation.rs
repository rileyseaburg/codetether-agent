//! Page navigation operations for native sessions.

#[path = "navigation_scripts.rs"]
mod scripts;

use crate::browser::{BrowserError, BrowserOutput, request::NavigationRequest};

/// Navigate the current tab to a URL.
///
/// # Errors
///
/// Returns [`BrowserError`] when the page cannot be loaded or the session is
/// not started.
pub(super) async fn goto(
    session: &super::super::BrowserSession,
    request: NavigationRequest,
) -> Result<BrowserOutput, BrowserError> {
    let html = super::fetch::html(&request.url).await?;
    let mut native = session.inner.native.lock().await;
    let slot = native
        .as_mut()
        .ok_or(BrowserError::SessionNotStarted)?
        .current_mut()?;
    let mut page = slot.page();
    page.goto_html(request.url, html);
    let report = scripts::run_and_report(&mut page);
    slot.replace(page);
    Ok(BrowserOutput::Json(report))
}

/// Navigate backward in the current tab history.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started.
pub(super) async fn back(
    session: &super::super::BrowserSession,
) -> Result<BrowserOutput, BrowserError> {
    let mut native = session.inner.native.lock().await;
    native
        .as_mut()
        .ok_or(BrowserError::SessionNotStarted)?
        .current_mut()?
        .session
        .back();
    Ok(super::lifecycle::ack())
}

/// Reload the current tab.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started.
pub(super) async fn reload(
    session: &super::super::BrowserSession,
) -> Result<BrowserOutput, BrowserError> {
    let mut native = session.inner.native.lock().await;
    let slot = native
        .as_mut()
        .ok_or(BrowserError::SessionNotStarted)?
        .current_mut()?;
    let mut page = slot.page();
    page.reload();
    let report = scripts::run_and_report(&mut page);
    slot.replace(page);
    Ok(BrowserOutput::Json(report))
}
