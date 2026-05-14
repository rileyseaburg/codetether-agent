//! File upload operations.

use crate::browser::{BrowserError, BrowserOutput, request::UploadRequest};
use std::path::Path;
use tetherscript::browser_agent::FilePayload;

/// Upload files through a file input selected by CSS.
///
/// # Errors
///
/// Returns [`BrowserError`] when the session is not started, a file is missing,
/// or the target element rejects the files.
pub(in crate::browser::session::native) async fn upload(
    session: &super::super::super::BrowserSession,
    request: UploadRequest,
) -> Result<BrowserOutput, BrowserError> {
    let mut files = Vec::with_capacity(request.paths.len());
    for path in &request.paths {
        files.push(file_payload(path).await?);
    }
    let mut native = session.inner.native.lock().await;
    let slot = native
        .as_mut()
        .ok_or(BrowserError::SessionNotStarted)?
        .current_mut()?;
    let mut page = slot.page();
    page.set_input_files(&super::selector::css(request.selector), files)
        .map_err(super::selector::js_error)?;
    slot.replace(page);
    Ok(super::super::lifecycle::ack())
}

async fn file_payload(path: &str) -> Result<FilePayload, BrowserError> {
    let bytes = tokio::fs::read(path)
        .await
        .map_err(|_| BrowserError::FileNotFound(path.into()))?;
    let name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(path);
    Ok(FilePayload::new(name, "application/octet-stream", bytes))
}
