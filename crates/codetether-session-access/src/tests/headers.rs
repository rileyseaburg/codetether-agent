//! Caller token fixture for workspace authority tests.
use axum::http::HeaderMap;

pub(super) fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("authorization", "Bearer test-caller".parse().unwrap());
    headers
}
