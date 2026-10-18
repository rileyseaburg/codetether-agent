//! Session-scoped human goal controls over authenticated HTTP.
mod error;
mod handlers;
mod host;
pub use host::GoalHost;
/// Build session goal routes under caller-owned auth and policy middleware.
/// # Returns
/// GET and POST handlers for the session's native goal.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_session_http::GoalHost>() {
/// let _routes = codetether_session_http::goal_router::<H, ()>();
/// # }
/// ```
pub fn router<H: GoalHost, S: Clone + Send + Sync + 'static>() -> axum::Router<S> {
    axum::Router::new().route(
        "/api/session/{id}/goal",
        axum::routing::get(handlers::get::<H>).post(handlers::post::<H>),
    )
}
#[cfg(test)]
mod tests;
