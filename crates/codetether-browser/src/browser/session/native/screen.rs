//! Screenshot capture for native browser sessions.

use crate::browser::{
    BrowserError, BrowserOutput, output::ScreenshotData, request::ScreenshotRequest,
};
use tetherscript::browser_agent::Locator;

/// Capture a raster screenshot of the current page.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started.
pub(super) async fn capture(
    session: &super::super::BrowserSession,
    request: ScreenshotRequest,
) -> Result<BrowserOutput, BrowserError> {
    let native = session.inner.native.lock().await;
    let page = native
        .as_ref()
        .ok_or(BrowserError::SessionNotStarted)?
        .current()?
        .page();
    let bytes = match request.selector {
        Some(selector) => page
            .element_screenshot(&Locator::css(selector).relaxed())
            .map(|shot| shot.image.to_ppm()),
        None => page.screenshot_ppm(),
    }
    .map_err(BrowserError::OperationFailed)?;
    Ok(BrowserOutput::Screenshot(ScreenshotData { bytes }))
}
