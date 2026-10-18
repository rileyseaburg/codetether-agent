//! Dispatch extraction over enqueue-before-notify task acceptance.
use super::HttpState;
use crate::dispatch::{
    contract::{DispatchTaskRequest, DispatchTaskResponse},
    enqueue,
};
use axum::{Json, extract::State};

pub(super) async fn dispatch_task<S: HttpState>(
    State(state): State<S>,
    Json(request): Json<DispatchTaskRequest>,
) -> Json<DispatchTaskResponse> {
    Json(enqueue(state.queue(), state.bus(), request).await)
}
