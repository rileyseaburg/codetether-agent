//! Worker route helper modules.

use codetether_mux_http as mux_realtime;

pub(super) fn router() -> axum::Router<super::AppState> {
    use axum::routing::{get, post};

    axum::Router::new()
        .merge(codetether_k8s_api::http::worker_routes::<super::AppState>())
        .route(
            "/v1/worker/tasks/stream",
            get(worker_task_stream::<super::AppState>),
        )
        .route(
            "/v1/worker/tasks/claim",
            post(worker_task_claim::<super::AppState>),
        )
        .route(
            "/v1/worker/tasks/release",
            post(worker_task_release::<super::AppState>),
        )
        .merge(mux_realtime::router())
}

pub(super) use codetether_task_api::worker_http::{
    worker_task_claim, worker_task_release, worker_task_stream,
};
