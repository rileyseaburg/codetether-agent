//! Native bindings and goal/fork routes alongside shared persisted-session HTTP.
#[path = "session_routes/host.rs"]
mod host;

#[cfg(test)]
#[path = "session_routes/fork.rs"]
mod fork;
#[path = "session_routes/fork_host.rs"]
mod fork_host;
#[path = "session_routes/goal_host.rs"]
mod goal_host;

/// Build the non-realtime session HTTP surface.
pub(super) fn router() -> axum::Router<super::AppState> {
    axum::Router::new()
        .merge(codetether_session_http::fork_router::<
            crate::session::Session,
            super::AppState,
        >())
        .merge(codetether_session_http::goal_router::<
            crate::session::Session,
            super::AppState,
        >())
        .merge(codetether_session_http::router::<
            crate::session::Session,
            super::AppState,
        >())
}
