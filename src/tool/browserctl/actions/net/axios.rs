//! `axios` tool action — replays HTTP via the page's axios instance.

use super::super::super::helpers::require_string;
use super::super::super::input::BrowserCtlInput;
use crate::browser::{BrowserCommand, BrowserError, BrowserOutput, request::AxiosRequest};

/// Build an [`AxiosRequest`] from raw input and dispatch it.
///
/// Prefers `json_body` (parsed object) but accepts `body` (JSON string)
/// so the payload captured by `network_log` can be replayed verbatim.
///
/// # Errors
///
/// Returns [`BrowserError::OperationFailed`] if `url` is missing,
/// or propagates browser errors from the command executor.

pub(in crate::tool::browserctl) async fn axios(
    input: &BrowserCtlInput,
) -> Result<BrowserOutput, BrowserError> {
    let url = require_string(&input.url, "url")
        .map_err(|e| BrowserError::OperationFailed(e.to_string()))?
        .to_string();
    let body = input.json_body.clone().or_else(|| {
        input
            .body
            .as_deref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
    });
    let request = AxiosRequest {
        method: input.method.clone().unwrap_or_else(|| "GET".to_string()),
        url,
        headers: input.headers.clone(),
        body,
        axios_path: input.axios_path.clone(),
    };
    super::super::execute(input, BrowserCommand::Axios(request)).await
}
