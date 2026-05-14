//! Browserctl action adapters.
//!
//! Action modules translate parsed tool input into typed browser commands.

use super::input::BrowserCtlInput;
use crate::browser::{BrowserCommand, BrowserError, BrowserOutput, browser_service};

/// Execute a typed browser command through the shared browser service.
///
/// # Errors
///
/// Returns [`BrowserError`] when the browser backend rejects the command.
pub(super) async fn execute(
    _input: &BrowserCtlInput,
    command: BrowserCommand,
) -> Result<BrowserOutput, BrowserError> {
    browser_service().session().execute(command).await
}

/// Device input action handlers.
pub(super) mod device;
/// DOM action handlers.
pub(super) mod dom;
/// DOM helper action handlers.
pub(super) mod dom_extra;
/// JavaScript evaluation action handlers.
pub(super) mod eval;
/// Lifecycle and screenshot action handlers.
pub(super) mod lifecycle;
/// Navigation action handlers.
pub(super) mod nav;
/// Network action handlers.
pub(super) mod net;
/// Tab action handlers.
pub(super) mod tabs;
/// Upload action handlers.
pub(super) mod upload;
