//! Parse, audit, dispatch and acknowledge a task-ingress event.
use crate::{CloudEventResponse, parse_cloud_event};
use axum::{
    Json,
    http::{HeaderMap, StatusCode},
};
use codetether_audit::{self as audit, AuditLog, AuditOutcome};
use codetether_task_queue::KnativeTaskQueue;

/// CloudEvent handler for Knative Eventing
/// Receives task events from Knative Broker and triggers execution
/// # Arguments
/// * `queue` - Shared in-process task queue.
/// * `log` - Audit log for received events.
/// * `headers` - Incoming CloudEvent or task headers.
/// * `body` - Incoming JSON event or task payload.
/// # Returns
/// An accepted acknowledgement containing the normalized event ID.
/// # Errors
/// Returns HTTP bad request when the shared event parser rejects input.
/// # Examples
/// ```no_run
/// # async fn example(queue: &codetether_task_queue::KnativeTaskQueue, log: &codetether_audit::AuditLog) {
/// let _ = codetether_task_ingest::receive_task_event(queue, log, axum::http::HeaderMap::new(), serde_json::json!({"task_id":"t1"})).await;
/// # }
/// ```
pub async fn receive_task_event(
    queue: &KnativeTaskQueue,
    log: &AuditLog,
    headers: HeaderMap,
    body: serde_json::Value,
) -> Result<Json<CloudEventResponse>, (StatusCode, String)> {
    let event =
        parse_cloud_event(&headers, body).map_err(|error| (StatusCode::BAD_REQUEST, error))?;
    tracing::info!(
        event_type = %event.event_type,
        event_id = %event.id,
        "Received CloudEvent from Knative"
    );

    // Log the incoming event for audit
    log.log(
        audit::AuditCategory::Api,
        format!("cloudevents:{}", event.event_type),
        AuditOutcome::Success,
        None,
        None,
    )
    .await;

    crate::dispatch::apply(queue, &event).await;

    Ok(Json(CloudEventResponse {
        status: "accepted".to_string(),
        event_id: event.id,
    }))
}
