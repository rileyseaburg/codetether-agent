//! agent routes for the task HTTP surface.
use super::{HttpState, agent, output};
use axum::{Router, routing::get};
/// Build this task route group over the application queue and bus.
/// # Returns
/// Routes without additional authentication or policy layers.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_task_api::http::HttpState>() {
/// let routes = codetether_task_api::http::agent_routes::<S>();
/// # }
/// ```
pub fn agent_routes<S: HttpState>() -> Router<S> {
    Router::new()
        .route(
            "/v1/agent/tasks",
            get(agent::list_agent_tasks::<S>).post(agent::create_agent_task::<S>),
        )
        .route("/v1/agent/tasks/{task_id}", get(agent::get_agent_task::<S>))
        .route(
            "/v1/agent/tasks/{task_id}/output",
            get(output::get_agent_task_output::<S>).post(output::agent_task_output::<S>),
        )
        .route(
            "/v1/agent/tasks/{task_id}/output/stream",
            get(output::stream_agent_task_output::<S>),
        )
}
