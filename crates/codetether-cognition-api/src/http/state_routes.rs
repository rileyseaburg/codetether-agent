//! Cognition state routes assembly.
use super::{HttpState, beliefs, governance, workspace};
use axum::{
    Router,
    routing::{get, post},
};
pub(super) fn router<S: HttpState>() -> Router<S> {
    Router::new()
        // Belief, attention, governance, workspace APIs
        .route("/v1/cognition/beliefs", get(beliefs::list_beliefs::<S>))
        .route("/v1/cognition/beliefs/{id}", get(beliefs::get_belief::<S>))
        .route(
            "/v1/cognition/attention",
            get(workspace::list_attention::<S>),
        )
        .route(
            "/v1/cognition/proposals",
            get(governance::list_proposals::<S>),
        )
        .route(
            "/v1/cognition/proposals/{id}/approve",
            post(governance::approve_proposal::<S>),
        )
        .route(
            "/v1/cognition/receipts",
            get(governance::list_receipts::<S>),
        )
        .route(
            "/v1/cognition/workspace",
            get(workspace::get_workspace::<S>),
        )
        .route(
            "/v1/cognition/governance",
            get(governance::get_governance::<S>),
        )
        .route(
            "/v1/cognition/personas/{id}",
            get(workspace::get_persona::<S>),
        )
}
