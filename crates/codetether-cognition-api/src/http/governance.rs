//! Cognition governance HTTP handlers.
use super::{HttpState, Proposals, Workspace};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use codetether_cognition_core::Proposal;
use codetether_cognition_core::SwarmGovernance;
use codetether_cognition_core::execution::DecisionReceipt;

pub(super) async fn list_proposals<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<Vec<Proposal>>, (StatusCode, String)> {
    crate::list_proposals(state.runtime().get_proposals().await)
}

pub(super) async fn approve_proposal<S: HttpState>(
    State(state): State<S>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    state
        .runtime()
        .approve_proposal(&id)
        .await
        .map(|_| Json(serde_json::json!({ "approved": true, "proposal_id": id })))
        .map_err(S::map_error)
}

pub(super) async fn list_receipts<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<Vec<DecisionReceipt>>, (StatusCode, String)> {
    Ok(Json(state.runtime().get_receipts().await))
}

pub(super) async fn get_governance<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<SwarmGovernance>, (StatusCode, String)> {
    Ok(Json(state.runtime().get_governance().await))
}
