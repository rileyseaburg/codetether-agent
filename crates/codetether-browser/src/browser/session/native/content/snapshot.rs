//! Structured snapshot generation for native browser pages.

use crate::browser::{
    BrowserError, BrowserOutput,
    output::{PageSnapshot, Viewport},
};

/// Return a snapshot of the current page state.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started.
pub(in crate::browser::session::native) async fn snapshot(
    session: &super::super::super::BrowserSession,
) -> Result<BrowserOutput, BrowserError> {
    let native = session.inner.native.lock().await;
    let page = native
        .as_ref()
        .ok_or(BrowserError::SessionNotStarted)?
        .current()?;
    Ok(BrowserOutput::Snapshot(PageSnapshot {
        url: page.session.url.clone(),
        title: super::title(page),
        text: super::title::document_text(page),
        viewport: Some(Viewport {
            width: page.viewport_width as u32,
            height: page.viewport_height as u32,
        }),
    }))
}
