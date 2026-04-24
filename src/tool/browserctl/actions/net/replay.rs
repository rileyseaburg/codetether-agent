//! `replay` tool action — re-fires a captured network_log entry.

use super::super::super::helpers::require_string;
use super::super::super::input::BrowserCtlInput;
use crate::browser::{BrowserCommand, BrowserError, BrowserOutput, request::ReplayRequest};

/// Build a [`ReplayRequest`] from raw input and dispatch it.
///
/// # Errors
///
/// Returns [`BrowserError::OperationFailed`] if `url_contains` is
/// missing; propagates browser errors from the executor.
pub(in crate::tool::browserctl) async fn replay(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let url_contains = require_string(&input.url_contains, "url_contains")
        .map_err(|e| BrowserError::OperationFailed(e.to_string()))?
        .to_string();
    let request = ReplayRequest {
        url_contains,
        method_filter: input.method.clone(),
        url_override: input.url.clone(),
        method_override: None,
        body_patch: input.body_patch.clone(),
        body_override: input.body.clone(),
        extra_headers: input.headers.clone(),
        with_credentials: input.with_credentials,
    };
    super::super::execute(input, BrowserCommand::Replay(request)).await
}
