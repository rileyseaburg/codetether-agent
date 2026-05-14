//! Tab close operation.

use crate::browser::{BrowserError, BrowserOutput, output::Ack, request::CloseTabRequest};

/// Close a tab by index.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started or the tab is not
/// found.
pub(in crate::browser::session::native) async fn close(
    session: &super::super::super::BrowserSession,
    request: CloseTabRequest,
) -> Result<BrowserOutput, BrowserError> {
    let mut native = session.inner.native.lock().await;
    let runtime = native.as_mut().ok_or(BrowserError::SessionNotStarted)?;
    runtime.close(request.index.unwrap_or(runtime.current))?;
    Ok(BrowserOutput::Ack(Ack { ok: true }))
}
