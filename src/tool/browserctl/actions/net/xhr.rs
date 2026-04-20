//! `xhr` tool action — replays HTTP via raw XMLHttpRequest.

use super::super::super::helpers::require_string;
use super::super::super::input::BrowserCtlInput;
use crate::browser::{BrowserCommand, BrowserError, BrowserOutput, request::XhrRequest};

/// Build a [`XhrRequest`] from raw input and dispatch it.
///
/// # Errors
///
/// Returns [`BrowserError::OperationFailed`] if `url` is missing,
/// or propagates browser errors from the command executor.

pub(in crate::tool::browserctl) async fn xhr(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let url = require_string(&input.url, "url")
        .map_err(|e| BrowserError::OperationFailed(e.to_string()))?
        .to_string();
    let request = XhrRequest {
        method: input.method.clone().unwrap_or_else(|| "GET".to_string()),
        url,
        headers: input.headers.clone(),
        body: input.body.clone(),
        with_credentials: input.with_credentials,
    };
    super::super::execute(input, BrowserCommand::Xhr(request)).await
}
