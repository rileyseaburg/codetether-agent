//! HTTP boundary for non-destructive edited-conversation branching.
mod error;
mod handler;
mod request;
/// Build the fork endpoint under caller-owned authentication and policy.
/// # Returns
/// POST /api/session/{id}/fork backed by the supplied session type.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_session_fork::ForkSession + serde::Serialize>() {
/// let _router = codetether_session_http::fork_router::<H, ()>();
/// # }
/// ```
pub fn router<
    H: codetether_session_fork::ForkSession + serde::Serialize + 'static,
    S: Clone + Send + Sync + 'static,
>() -> axum::Router<S> {
    axum::Router::new().route(
        "/api/session/{id}/fork",
        axum::routing::post(handler::fork::<H>),
    )
}
