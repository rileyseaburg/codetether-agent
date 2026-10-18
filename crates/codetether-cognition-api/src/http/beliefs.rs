//! Cognition beliefs HTTP handlers.
use super::{Beliefs, HttpState};
use crate::BeliefFilter;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use codetether_cognition_core::beliefs::Belief;

pub(super) async fn list_beliefs<S: HttpState>(
    State(state): State<S>,
    Query(filter): Query<BeliefFilter>,
) -> Result<Json<Vec<Belief>>, (StatusCode, String)> {
    crate::list_beliefs(state.runtime().get_beliefs().await, filter)
}

pub(super) async fn get_belief<S: HttpState>(
    State(state): State<S>,
    Path(id): Path<String>,
) -> Result<Json<Belief>, (StatusCode, String)> {
    crate::belief(state.runtime().get_belief(&id).await, &id)
}
