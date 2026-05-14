//! Shared page mutation helper for device operations.

use crate::browser::{BrowserError, BrowserOutput};
use tetherscript::browser_agent::BrowserPage;

/// Run a synchronous device operation against the current page.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or the operation
/// fails.
pub(super) async fn with<F>(
    session: &super::super::super::BrowserSession,
    op: F,
) -> Result<BrowserOutput, BrowserError>
where
    F: FnOnce(&mut BrowserPage) -> Result<(), BrowserError>,
{
    let mut native = session.inner.native.lock().await;
    let slot = native
        .as_mut()
        .ok_or(BrowserError::SessionNotStarted)?
        .current_mut()?;
    let mut page = slot.page();
    let result = op(&mut page);
    slot.replace(page);
    result?;
    Ok(super::super::lifecycle::ack())
}
