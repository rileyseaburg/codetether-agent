//! Native bindings and goal/fork routes alongside shared persisted-session HTTP.
#[path = "session_routes/host.rs"]
mod host;

#[path = "session_routes/fork.rs"]
mod fork;
#[path = "session_routes/goal.rs"]
mod goal;
#[cfg(test)]
#[path = "session_routes/goal_tests.rs"]
mod goal_tests;

/// Build the non-realtime session HTTP surface.
pub(super) fn router() -> axum::Router<super::AppState> {
    use axum::routing::{get, post};

    axum::Router::new()
        .route("/api/session/{id}/goal", get(goal::get).post(goal::post))
        .route("/api/session/{id}/fork", post(fork::fork))
        .merge(codetether_session_http::router::<
            crate::session::Session,
            super::AppState,
        >())
}
