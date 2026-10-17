//! CloudEvent task ingestion and delivery.
use super::*;

/// Receive CloudEvents POST (for Knative integration)
/// This endpoint receives tasks pushed via Knative Eventing
pub(super) async fn receive_task(
    State(state): State<WorkerServerState>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> StatusCode {
    let event = match parse_cloud_event(&headers, payload) {
        Ok(event) => event,
        Err(error) => {
            tracing::warn!(target: "codetether_agent::worker_server", "Rejected task event: {}", error);
            return StatusCode::BAD_REQUEST;
        }
    };

    let task_id = event
        .data
        .get("task_id")
        .or_else(|| event.data.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or(event.id.as_str());

    tracing::info!(target: "codetether_agent::worker_server",
        "Received task via CloudEvent: {} ({})",
        task_id,
        event.event_type
    );

    if !state.notify_new_task(task_id).await {
        return StatusCode::SERVICE_UNAVAILABLE;
    }

    StatusCode::ACCEPTED
}
