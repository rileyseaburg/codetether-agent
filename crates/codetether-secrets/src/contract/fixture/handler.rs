//! Local Vault request dispatcher.
use super::state::Calls;
use axum::{
    Json,
    body::{Body, to_bytes},
    extract::{Request, State},
    http::StatusCode,
};
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub(super) async fn handle(
    State(state): State<Arc<Mutex<Calls>>>,
    request: Request<Body>,
) -> (StatusCode, Json<Value>) {
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    if path.ends_with("/lookup-self") {
        return (StatusCode::OK, Json(crate::contract::response::lookup()));
    }
    let bytes = to_bytes(request.into_body(), 1024 * 1024).await.unwrap();
    let body = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    let mut state = state.lock().unwrap();
    state.requests.push((method, path, body));
    let (status, body) = state.replies.pop_front().expect("unexpected Vault request");
    (
        StatusCode::from_u16(status).unwrap(),
        Json(if status == 200 {
            crate::contract::response::envelope(body)
        } else {
            body
        }),
    )
}
