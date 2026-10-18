//! Cognition workspace HTTP handlers.
use super::{HttpState, Personas, Workspace};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use codetether_cognition_core::AttentionItem;
use codetether_cognition_core::GlobalWorkspace;
use codetether_cognition_core::PersonaRuntimeState;
use codetether_cognition_core::status::LineageGraph;

pub(super) async fn get_swarm_lineage<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<LineageGraph>, (StatusCode, String)> {
    Ok(Json(state.runtime().lineage_graph().await))
}

pub(super) async fn list_attention<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<Vec<AttentionItem>>, (StatusCode, String)> {
    Ok(Json(state.runtime().get_attention_queue().await))
}

pub(super) async fn get_workspace<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<GlobalWorkspace>, (StatusCode, String)> {
    Ok(Json(state.runtime().get_workspace().await))
}

pub(super) async fn get_persona<S: HttpState>(
    State(state): State<S>,
    Path(id): Path<String>,
) -> Result<Json<PersonaRuntimeState>, (StatusCode, String)> {
    crate::persona(state.runtime().get_persona(&id).await, &id)
}
