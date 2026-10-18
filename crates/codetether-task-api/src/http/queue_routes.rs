//! queue routes for the task HTTP surface.
use super::{HttpState, queue};
use axum::{
    Router,
    routing::{get, post},
};
/// Build this task route group over the application queue and bus.
/// # Returns
/// Routes without additional authentication or policy layers.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_task_api::http::HttpState>() {
/// let routes = codetether_task_api::http::queue_routes::<S>();
/// # }
/// ```
pub fn queue_routes<S: HttpState>() -> Router<S> {
    Router::new()
        .route("/v1/knative/tasks", get(queue::list_knative_tasks::<S>))
        .route(
            "/v1/knative/tasks/{task_id}",
            get(queue::get_knative_task::<S>),
        )
        .route(
            "/v1/knative/tasks/{task_id}/claim",
            post(queue::claim_knative_task::<S>),
        )
        .route(
            "/v1/knative/tasks/{task_id}/complete",
            post(queue::complete_knative_task::<S>),
        )
}
