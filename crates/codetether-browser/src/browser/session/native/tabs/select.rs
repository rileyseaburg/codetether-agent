//! Tab selection operation.

use crate::browser::{BrowserError, BrowserOutput, output::Ack, request::TabSelectRequest};

/// Select a tab by index.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or the tab is not
/// found.
pub(in crate::browser::session::native) async fn select(
    session: &super::super::super::BrowserSession,
    request: TabSelectRequest,
) -> Result<BrowserOutput, BrowserError> {
    session
        .inner
        .native
        .lock()
        .await
        .as_mut()
        .ok_or(BrowserError::SessionNotStarted)?
        .select(request.index)?;
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}
