use crate::browser::{BrowserError, BrowserOutput, request::ReplayRequest};

pub(super) async fn replay(
    _session: &super::super::super::BrowserSession,
    request: ReplayRequest,
) -> Result<BrowserOutput, BrowserError> {
    let method = request
        .method_override
        .or(request.method_filter)
        .unwrap_or_else(|| "GET".into());
    let url = request.url_override.unwrap_or(request.url_contains);
    let body = request
        .body_override
        .or_else(|| request.body_patch.map(|value| value.to_string()));
    super::http::send(&method, &url, request.extra_headers, body).await
}
