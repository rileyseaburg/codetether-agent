//! `fetch` tool action — replays HTTP via the page's fetch() API.

use super::super::super::helpers::require_string;
use super::super::super::input::BrowserCtlInput;
use crate::browser::{BrowserCommand, BrowserError, BrowserOutput, request::FetchRequest};

/// Build a [`FetchRequest`] from raw input and dispatch it.
///
/// # Errors
///
/// Returns [`BrowserError::OperationFailed`] if `url` is missing,
/// or propagates browser errors from the command executor.

pub(in crate::tool::browserctl) async fn fetch(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let url = require_string(&input.url, "url")
        .map_err(|e| BrowserError::OperationFailed(e.to_string()))?
        .to_string();
    let request = FetchRequest {
        method: input.method.clone().unwrap_or_else(|| "GET".to_string()),
        url,
        headers: input.headers.clone(),
        body: input.body.clone(),
        credentials: input.credentials.clone(),
    };
    super::super::execute(input, BrowserCommand::Fetch(request)).await
}
