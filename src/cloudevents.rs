//! Axum header adaptation for the shared task-ingress parser.
use axum::http::HeaderMap;
pub use codetether_a2a_worker_core::cloudevents::CloudEvent;
use serde_json::Value;

/// Parse a task-ingress event using Axum headers and an owned JSON body.
///
/// # Arguments
/// * `headers` - Incoming request headers.
/// * `body` - Incoming JSON event or task payload.
///
/// # Returns
/// The normalized event.
///
/// # Errors
/// Returns the shared parser error for invalid event/task input.
///
/// # Examples
/// ```
/// use codetether_agent::cloudevents::parse_cloud_event;
/// let event = parse_cloud_event(&axum::http::HeaderMap::new(), serde_json::json!({"task_id":"t1"})).unwrap();
/// assert_eq!(event.event_type, "codetether.task.created");
/// ```
pub fn parse_cloud_event(headers: &HeaderMap, body: Value) -> Result<CloudEvent, String> {
    codetether_a2a_worker_core::cloudevents::parse_cloud_event(|name| header(headers, name), body)
}

fn header(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}
