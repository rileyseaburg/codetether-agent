//! Browserctl navigation lifecycle action adapters.

use crate::input::BrowserCtlInput;
use codetether_browser::{BrowserCommand, BrowserError, BrowserOutput, request::StartRequest};

/// Execute a health command.
///
/// # Errors
///
/// Returns [`BrowserError`] when execution fails.
pub(crate) async fn health(input: &BrowserCtlInput) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Health).await
}

/// Execute a start command.
///
/// # Errors
///
/// Returns [`BrowserError`] when execution fails.
pub(crate) async fn start(input: &BrowserCtlInput) -> Result<BrowserOutput, BrowserError> {
    let request = StartRequest {
        headless: input.headless.unwrap_or(true),
        executable_path: input.executable_path.clone(),
        user_data_dir: input.user_data_dir.clone(),
        ws_url: input.ws_url.clone(),
    };
    super::super::execute(input, BrowserCommand::Start(request)).await
}

/// Execute a stop command.
///
/// # Errors
///
/// Returns [`BrowserError`] when execution fails.
pub(crate) async fn stop(input: &BrowserCtlInput) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Stop).await
}

/// Execute a snapshot command.
///
/// # Errors
///
/// Returns [`BrowserError`] when execution fails.
pub(crate) async fn snapshot(input: &BrowserCtlInput) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Snapshot).await
}
