//! Local Vault endpoint behavior with secret-safe assertion messages.
use super::{counts::Counts, payloads};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use std::sync::{Arc, atomic::Ordering};

pub(super) async fn lookup(
    State(state): State<Arc<Counts>>,
    headers: HeaderMap,
) -> (StatusCode, Json<serde_json::Value>) {
    assert!(headers["X-Vault-Token"] == "fixture-token-not-a-secret");
    state.lookups.fetch_add(1, Ordering::SeqCst);
    if state.lookup_denied {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"errors":["denied"]})),
        );
    }
    (
        StatusCode::OK,
        Json(payloads::lookup(state.renewable, state.ttl)),
    )
}

pub(super) async fn renew(
    State(state): State<Arc<Counts>>,
    headers: HeaderMap,
) -> (StatusCode, Json<serde_json::Value>) {
    assert!(headers["X-Vault-Token"] == "fixture-token-not-a-secret");
    let call = state.renewals.fetch_add(1, Ordering::SeqCst);
    let status = if call == 0 { state.renew_status } else { 200 };
    if status != 200 {
        return (
            StatusCode::from_u16(status).unwrap(),
            Json(serde_json::json!({"errors":["fixture failure"]})),
        );
    }
    (
        StatusCode::OK,
        Json(payloads::renewal(state.renewable, state.ttl)),
    )
}
