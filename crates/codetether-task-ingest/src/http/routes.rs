//! Task-ingress HTTP route assembly.
use super::{HttpState, handler};
use axum::{Router, routing::post};

/// Build the task-ingress route using the caller's queue and audit log.
/// # Returns
/// The task event POST route ready to merge beneath the server's middleware.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_task_ingest::http::HttpState>() {
/// let routes = codetether_task_ingest::http::router::<S>();
/// # }
/// ```
pub fn router<S: HttpState>() -> Router<S> {
    Router::new().route("/task", post(handler::receive_task_event::<S>))
}
