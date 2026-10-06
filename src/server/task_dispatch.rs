//! HTTP adapter for local queue acceptance; not executor acknowledgement.

mod contract;
mod service;
#[cfg(test)]
mod tests;

use super::AppState;
use axum::{Json, extract::State};
use contract::{DispatchTaskRequest, DispatchTaskResponse};

pub(super) async fn dispatch_task(
    State(state): State<AppState>,
    Json(request): Json<DispatchTaskRequest>,
) -> Json<DispatchTaskResponse> {
    Json(service::enqueue(&state.knative_tasks, &state.bus, request).await)
}
