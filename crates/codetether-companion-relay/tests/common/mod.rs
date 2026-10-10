//! Shared HTTP harness for relay contract tests.
use axum::body::{Body, to_bytes};
use axum::http::Request;
use codetether_companion_relay::{Analysis, Analyze, Relay, router};
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

pub const OWNER: &str = "synthetic-owner-credential-not-a-real-secret";

/// Router with a synthetic analyzer that streams one delta.
pub fn app() -> axum::Router {
    let analyze: Analyze = Arc::new(|mut input: Analysis| {
        Box::pin(async move {
            (input.delta)("seen");
            Ok(())
        })
    });
    router(Relay::new(OWNER, analyze, "https://server.codetether.run", None).unwrap())
}
/// Send a JSON request and return status plus parsed body.
pub async fn call(
    app: &axum::Router,
    method: &str,
    path: &str,
    auth: Option<&str>,
    body: Option<Value>,
) -> (u16, Value) {
    let mut request = Request::builder().method(method).uri(path);
    if let Some(token) = auth {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    let body = body.map_or_else(Body::empty, |v| Body::from(v.to_string()));
    let request = request
        .header("content-type", "application/json")
        .body(body)
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status().as_u16();
    let bytes = to_bytes(response.into_body(), 1 << 20).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}
