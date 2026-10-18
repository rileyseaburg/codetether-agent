//! dispatch routes for the task HTTP surface.
use super::{HttpState, dispatch};
use axum::{Router, routing::post};
/// Build this task route group over the application queue and bus.
/// # Returns
/// Routes without additional authentication or policy layers.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_task_api::http::HttpState>() {
/// let routes = codetether_task_api::http::dispatch_routes::<S>();
/// # }
/// ```
pub fn dispatch_routes<S: HttpState>() -> Router<S> {
    Router::new().route("/v1/tasks/dispatch", post(dispatch::dispatch_task::<S>))
}
