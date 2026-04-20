//! Network inspection & replay browserctl actions.

use super::super::helpers::require_string;
use super::super::input::BrowserCtlInput;
use crate::browser::{
    BrowserCommand,
    request::{AxiosRequest, DiagnoseRequest, FetchRequest, NetworkLogRequest, XhrRequest},
};

pub(in crate::tool::browserctl) async fn network_log(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let request = NetworkLogRequest {
        limit: input.limit,
        url_contains: input
            .url_contains
            .as_deref()
            .filter(|v| !v.trim().is_empty())
            .map(str::to_string),
        method: input
            .method
            .as_deref()
            .filter(|v| !v.trim().is_empty())
            .map(str::to_string),
    };
    super::execute(input, BrowserCommand::NetworkLog(request)).await
}

pub(in crate::tool::browserctl) async fn fetch(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let url = require_string(&input.url, "url")
        .map_err(|e| crate::browser::BrowserError::OperationFailed(e.to_string()))?
        .to_string();
    let method = input
        .method
        .clone()
        .unwrap_or_else(|| "GET".to_string());
    let request = FetchRequest {
        method,
        url,
        headers: input.headers.clone(),
        body: input.body.clone(),
        credentials: input.credentials.clone(),
    };
    super::execute(input, BrowserCommand::Fetch(request)).await
}

pub(in crate::tool::browserctl) async fn axios(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let url = require_string(&input.url, "url")
        .map_err(|e| crate::browser::BrowserError::OperationFailed(e.to_string()))?
        .to_string();
    let method = input
        .method
        .clone()
        .unwrap_or_else(|| "GET".to_string());
    // Prefer `json_body` (parsed object) but accept `body` (JSON string) so
    // the same payload captured by network_log can be replayed verbatim.
    let body = input.json_body.clone().or_else(|| {
        input
            .body
            .as_deref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
    });
    let request = AxiosRequest {
        method,
        url,
        headers: input.headers.clone(),
        body,
        axios_path: input.axios_path.clone(),
    };
    super::execute(input, BrowserCommand::Axios(request)).await
}

pub(in crate::tool::browserctl) async fn diagnose(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    super::execute(input, BrowserCommand::Diagnose(DiagnoseRequest {})).await
}

pub(in crate::tool::browserctl) async fn xhr(
    input: &BrowserCtlInput,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let url = require_string(&input.url, "url")
        .map_err(|e| crate::browser::BrowserError::OperationFailed(e.to_string()))?
        .to_string();
    let method = input.method.clone().unwrap_or_else(|| "GET".to_string());
    let request = XhrRequest {
        method,
        url,
        headers: input.headers.clone(),
        body: input.body.clone(),
        with_credentials: input.with_credentials,
    };
    super::execute(input, BrowserCommand::Xhr(request)).await
}
