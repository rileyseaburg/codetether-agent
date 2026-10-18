//! Persisted-session creation, retrieval, listing and blocking prompts over HTTP.
//!
//! Implement SessionHost for native storage, then mount router under the
//! application's authentication and policy middleware.
mod create;
mod get;
mod host;
mod list;
mod prompt;
pub use host::SessionHost;
/// Build persisted-session routes backed by the supplied native session type.
/// # Returns
/// Create/list/get/prompt routes with the existing HTTP response policy.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_session_http::SessionHost>() {
/// let _router = codetether_session_http::router::<H, ()>();
/// # }
/// ```
pub fn router<H: SessionHost, S: Clone + Send + Sync + 'static>() -> axum::Router<S> {
    use axum::routing::{get, post};
    axum::Router::new()
        .route(
            "/api/session",
            get(list::list::<H>).post(create::create::<H>),
        )
        .route("/api/session/{id}", get(get::get::<H>))
        .route("/api/session/{id}/prompt", post(prompt::prompt::<H>))
}

mod goal;
pub use goal::{GoalHost, router as goal_router};
