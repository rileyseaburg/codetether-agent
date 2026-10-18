//! Cognition persona create HTTP handlers.
use super::{HttpState, Personas};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use codetether_cognition_core::CreatePersonaRequest;
use codetether_cognition_core::PersonaRuntimeState;
use codetether_cognition_core::SpawnPersonaRequest;

pub(super) async fn create_persona<S: HttpState>(
    State(state): State<S>,
    Json(req): Json<CreatePersonaRequest>,
) -> Result<Json<PersonaRuntimeState>, (StatusCode, String)> {
    state
        .runtime()
        .create_persona(req)
        .await
        .map(Json)
        .map_err(S::map_error)
}

pub(super) async fn spawn_persona<S: HttpState>(
    State(state): State<S>,
    Path(id): Path<String>,
    Json(req): Json<SpawnPersonaRequest>,
) -> Result<Json<PersonaRuntimeState>, (StatusCode, String)> {
    state
        .runtime()
        .spawn_child(&id, req)
        .await
        .map(Json)
        .map_err(S::map_error)
}
