//! Worker route helper modules.

#[path = "mux_realtime.rs"]
mod mux_realtime;
#[path = "worker_stream.rs"]
mod worker_stream;
#[path = "worker_task_claim.rs"]
mod worker_task_claim;
#[path = "worker_task_release.rs"]
mod worker_task_release;
#[path = "worker_task_stream.rs"]
mod worker_task_stream;

pub(super) fn router() -> axum::Router<super::AppState> {
    use axum::routing::{get, post};

    axum::Router::new()
        .route("/v1/worker/connected", get(super::list_connected_workers))
        .route("/v1/agent/workers", get(super::list_connected_workers))
        .route("/v1/worker/tasks/stream", get(worker_task_stream))
        .route("/v1/worker/tasks/claim", post(worker_task_claim))
        .route("/v1/worker/tasks/release", post(worker_task_release))
        .merge(mux_realtime::router())
}

pub(super) use worker_task_claim::worker_task_claim;
pub(super) use worker_task_release::worker_task_release;
pub(super) use worker_task_stream::worker_task_stream;
