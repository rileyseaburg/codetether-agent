//! Worker route helper modules.

use codetether_mux_http as mux_realtime;

/// Build the existing worker status, task and mux route group.
/// # Returns
/// Routes to merge beneath the caller's authentication and policy middleware.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_k8s_api::http::HttpState + codetether_task_api::worker_http::WorkerState>() {
/// let _routes = codetether_server_router::worker_routes::<S>();
/// # }
/// ```
pub fn worker_routes<S>() -> axum::Router<S>
where
    S: codetether_k8s_api::http::HttpState + codetether_task_api::worker_http::WorkerState,
{
    use axum::routing::{get, post};

    axum::Router::new()
        .merge(codetether_k8s_api::http::worker_routes::<S>())
        .route("/v1/worker/tasks/stream", get(worker_task_stream::<S>))
        .route("/v1/worker/tasks/claim", post(worker_task_claim::<S>))
        .route("/v1/worker/tasks/release", post(worker_task_release::<S>))
        .merge(mux_realtime::router())
}

pub(super) use codetether_task_api::worker_http::{
    worker_task_claim, worker_task_release, worker_task_stream,
};
