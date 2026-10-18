//! Cognition persona reap HTTP handlers.
use super::{HttpState, Personas};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use codetether_cognition_core::ReapPersonaRequest;
use codetether_cognition_core::status::ReapPersonaResponse;

pub(super) async fn reap_persona<S: HttpState>(
    State(state): State<S>,
    Path(id): Path<String>,
    payload: Option<Json<ReapPersonaRequest>>,
) -> Result<Json<ReapPersonaResponse>, (StatusCode, String)> {
    let req = payload
        .map(|Json(body)| body)
        .unwrap_or(ReapPersonaRequest {
            cascade: Some(false),
            reason: None,
        });

    state
        .runtime()
        .reap_persona(&id, req)
        .await
        .map(Json)
        .map_err(S::map_error)
}
