//! Parse structured events or normalize binary and legacy task payloads.
use super::CloudEvent;
use serde_json::Value;

/// Parse a structured CloudEvent or normalize task data with binary headers.
///
/// # Arguments
/// * `header` - Lookup for one HTTP header as a Unicode string.
/// * `body` - Owned JSON body to inspect and retain as event data.
///
/// # Returns
/// The event using the existing structured-field precedence and legacy defaults.
///
/// # Errors
/// Returns a string when structured metadata cannot deserialize or no task/event ID exists.
///
/// # Examples
/// ```
/// use codetether_a2a_worker_core::cloudevents::parse_cloud_event;
/// let event = parse_cloud_event(|_| None, serde_json::json!({"task_id":"t1"})).unwrap();
/// assert_eq!(event.id, "legacy-task-event");
/// ```
pub fn parse_cloud_event(
    header: impl Fn(&str) -> Option<String>,
    body: Value,
) -> Result<CloudEvent, String> {
    if body.get("type").is_some() && body.get("source").is_some() && body.get("id").is_some() {
        return serde_json::from_value(body).map_err(|err| err.to_string());
    }
    if body.get("task_id").is_none() && body.get("id").is_none() && header("ce-id").is_none() {
        return Err("request body is not a CloudEvent or task payload".to_string());
    }
    Ok(CloudEvent {
        id: header("ce-id")
            .or_else(|| body.get("id").and_then(Value::as_str).map(str::to_string))
            .unwrap_or_else(|| "legacy-task-event".to_string()),
        source: header("ce-source").unwrap_or_else(|| "codetether:legacy".to_string()),
        event_type: header("ce-type").unwrap_or_else(|| "codetether.task.created".to_string()),
        timestamp: header("ce-time"),
        spec_version: header("ce-specversion").or(Some("1.0".to_string())),
        data: body,
    })
}
