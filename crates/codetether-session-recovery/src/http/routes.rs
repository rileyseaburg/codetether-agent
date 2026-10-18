//! Workspace-scoped durable session recovery route assembly.
use super::{HttpState, handler};
use crate::RecoverySession;
use axum::{Router, routing::post};
/// Build the authorized session recovery route.
/// # Returns
/// A recovery route for the supplied storage implementation and request state.
/// # Examples
/// ```no_run
/// # fn example<R: codetether_session_recovery::RecoverySession + 'static, S: codetether_session_recovery::http::HttpState>() {
/// let routes = codetether_session_recovery::http::router::<R, S>();
/// # }
/// ```
pub fn router<R: RecoverySession + 'static, S: HttpState>() -> Router<S> {
    Router::new().route(
        "/v1/agent/codebases/{codebase_id}/sessions/{session_id}/resume",
        post(handler::handle::<R, S>),
    )
}
