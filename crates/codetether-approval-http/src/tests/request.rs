//! Approval request fixture for the HTTP round trip.
use axum::{body::Body, http::Request};
use serde_json::json;
pub(super) fn approval_request() -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/v1/tools/approvals/request")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(
            json!({
                "tool": "bash", "action": "execute", "resource": "bash:abc"
            })
            .to_string(),
        ))
        .expect("request")
}
