//! `diagnose` tool action — dumps page HTTP plumbing for debugging.

use super::super::super::input::BrowserCtlInput;
use codetether_browser::{BrowserCommand, BrowserError, BrowserOutput, request::DiagnoseRequest};

/// Dispatch a [`DiagnoseRequest`] to dump service workers, axios
/// instances, CSP, and network-log summary.
///
/// # Errors
///
/// Propagates browser errors from the command executor.

pub(crate) async fn diagnose(input: &BrowserCtlInput) -> Result<BrowserOutput, BrowserError> {
    super::super::execute(input, BrowserCommand::Diagnose(DiagnoseRequest {})).await
}
