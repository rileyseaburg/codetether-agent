//! Normalized task-ingress CloudEvent record.
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// CloudEvent metadata and its task payload.
///
/// # Examples
/// ```
/// use codetether_a2a_worker_core::cloudevents::CloudEvent;
/// let event = CloudEvent { id: "e1".into(), source: "example".into(),
///     event_type: "task.created".into(), timestamp: None,
///     spec_version: Some("1.0".into()), data: serde_json::json!({}) };
/// assert_eq!(event.id, "e1");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudEvent {
    pub id: String,
    pub source: String,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "time")]
    pub timestamp: Option<String>,
    #[serde(rename = "specversion")]
    pub spec_version: Option<String>,
    #[serde(default)]
    pub data: Value,
}
