//! Cognition control HTTP handlers.
use super::{HttpState, Lifecycle};
use axum::{Json, extract::State, http::StatusCode};
use codetether_cognition_core::control::StartCognitionRequest;
use codetether_cognition_core::control::StopCognitionRequest;
use codetether_cognition_core::status::CognitionStatus;

pub(super) async fn start_cognition<S: HttpState>(
    State(state): State<S>,
    payload: Option<Json<StartCognitionRequest>>,
) -> Result<Json<CognitionStatus>, (StatusCode, String)> {
    state
        .runtime()
        .start(payload.map(|Json(body)| body))
        .await
        .map(Json)
        .map_err(S::map_error)
}

pub(super) async fn stop_cognition<S: HttpState>(
    State(state): State<S>,
    payload: Option<Json<StopCognitionRequest>>,
) -> Result<Json<CognitionStatus>, (StatusCode, String)> {
    let reason = payload.and_then(|Json(body)| body.reason);
    state
        .runtime()
        .stop(reason)
        .await
        .map(Json)
        .map_err(S::map_error)
}

pub(super) async fn get_cognition_status<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<CognitionStatus>, (StatusCode, String)> {
    Ok(Json(state.runtime().status().await))
}
