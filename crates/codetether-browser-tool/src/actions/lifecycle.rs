//! Browserctl wait and screenshot action adapters.

use super::super::helpers::require_string;
use super::super::input::BrowserCtlInput;
use codetether_browser::{
    BrowserCommand,
    request::{ScreenshotRequest, WaitRequest},
};

/// Build and execute a wait command.
///
/// # Errors
///
/// Returns [`BrowserError`] when execution fails.
pub(crate) async fn wait(
    input: &BrowserCtlInput,
) -> Result<codetether_browser::BrowserOutput, codetether_browser::BrowserError> {
    let request = WaitRequest {
        text: input.text.clone(),
        text_gone: input.text_gone.clone(),
        url_contains: input.url_contains.clone(),
        selector: input.selector.clone(),
        frame_selector: input.frame_selector.clone(),
        state: input.state.clone().unwrap_or_else(|| "visible".into()),
        timeout_ms: input.timeout_ms.unwrap_or(30_000),
    };
    super::execute(input, BrowserCommand::Wait(request)).await
}

/// Build and execute a screenshot command.
///
/// # Errors
///
/// Returns [`BrowserError`] when screenshot capture or file writing fails.
pub(crate) async fn screenshot(
    input: &BrowserCtlInput,
) -> Result<codetether_browser::BrowserOutput, codetether_browser::BrowserError> {
    let _ = require_string(&input.path, "path")?;
    let request = ScreenshotRequest {
        selector: input.selector.clone(),
        frame_selector: input.frame_selector.clone(),
        full_page: input.full_page.unwrap_or(true),
    };
    super::execute(input, BrowserCommand::Screenshot(request)).await
}
