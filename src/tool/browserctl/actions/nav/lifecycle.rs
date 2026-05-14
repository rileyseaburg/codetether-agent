//! Browserctl navigation lifecycle action adapters.

use crate::browser::{BrowserCommand, BrowserError, BrowserOutput, request::StartRequest};
use crate::tool::browserctl::input::BrowserCtlInput;

/// Execute a health command.
///
/// # Errors
///
/// Returns [`BrowserError`] when execution fails.
pub(in crate::tool::browserctl) async fn health(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Health).await
}

/// Execute a start command.
///
/// # Errors
///
/// Returns [`BrowserError`] when execution fails.
pub(in crate::tool::browserctl) async fn start(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
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
pub(in crate::tool::browserctl) async fn stop(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Stop).await
}

/// Execute a snapshot command.
///
/// # Errors
///
/// Returns [`BrowserError`] when execution fails.
pub(in crate::tool::browserctl) async fn snapshot(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Snapshot).await
}
