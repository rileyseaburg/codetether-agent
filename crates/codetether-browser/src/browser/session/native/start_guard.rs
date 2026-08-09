//! Guard rejecting real-browser start requests on the native backend.

use crate::browser::{BrowserError, request::StartRequest};

/// Rejects start requests that require an actual browser process.
///
/// # Errors
///
/// Returns [`BrowserError::NotImplemented`] when `ws_url` or
/// `executable_path` is set, since the native backend cannot honor either and
/// silently ignoring them yields a session that never executes page scripts.
pub(super) fn reject_real_browser_request(request: &StartRequest) -> Result<(), BrowserError> {
    if let Some(ws) = request.ws_url.as_deref() {
        return Err(unsupported("ws_url", ws));
    }
    if let Some(path) = request.executable_path.as_deref() {
        return Err(unsupported("executable_path", path));
    }
    Ok(())
}

fn unsupported(field: &str, value: &str) -> BrowserError {
    BrowserError::NotImplemented(format!(
        "native backend cannot use {field}={value}; it interprets JavaScript \
         instead of driving a browser process, so page scripts and event \
         handlers do not run. Use a real browser for runtime proof."
    ))
}

#[cfg(test)]
#[path = "start_guard_tests.rs"]
mod tests;
