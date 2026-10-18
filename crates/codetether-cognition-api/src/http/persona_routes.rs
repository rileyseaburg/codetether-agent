//! Cognition persona routes assembly.
use super::{HttpState, persona_create, persona_reap, workspace};
use axum::{
    Router,
    routing::{get, post},
};
pub(super) fn router<S: HttpState>() -> Router<S> {
    Router::new()
        // Swarm persona lifecycle APIs
        .route(
            "/v1/swarm/personas",
            post(persona_create::create_persona::<S>),
        )
        .route(
            "/v1/swarm/personas/{id}/spawn",
            post(persona_create::spawn_persona::<S>),
        )
        .route(
            "/v1/swarm/personas/{id}/reap",
            post(persona_reap::reap_persona::<S>),
        )
        .route("/v1/swarm/lineage", get(workspace::get_swarm_lineage::<S>))
}
