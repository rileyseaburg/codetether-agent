//! `network_log` tool action — queries the in-page captured request log.

use super::super::super::input::BrowserCtlInput;
use crate::browser::{BrowserCommand, request::NetworkLogRequest};

/// Build a [`NetworkLogRequest`] from raw input and dispatch it.
///
/// # Errors
///
/// Propagates browser errors from the command executor.

pub(in crate::tool::browserctl) async fn network_log(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let request = NetworkLogRequest {
        limit: input.limit,
        url_contains: trimmed(&input.url_contains),
        method: trimmed(&input.method),
    };
    super::super::execute(input, BrowserCommand::NetworkLog(request)).await
}

fn trimmed(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .filter(|v| !v.trim().is_empty())
        .map(str::to_string)
}
