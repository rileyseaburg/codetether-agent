use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

pub fn parse_cloud_event(headers: &HeaderMap, body: Value) -> Result<CloudEvent, String> {
    if body.get("type").is_some() && body.get("source").is_some() && body.get("id").is_some() {
        return serde_json::from_value(body).map_err(|err| err.to_string());
    }
    if body.get("task_id").is_none()
        && body.get("id").is_none()
        && header(headers, "ce-id").is_none()
    {
        return Err("request body is not a CloudEvent or task payload".to_string());
    }
    Ok(CloudEvent {
        id: header(headers, "ce-id")
            .or_else(|| body.get("id").and_then(Value::as_str).map(str::to_string))
            .unwrap_or_else(|| "legacy-task-event".to_string()),
        source: header(headers, "ce-source").unwrap_or_else(|| "codetether:legacy".to_string()),
        event_type: header(headers, "ce-type")
            .unwrap_or_else(|| "codetether.task.created".to_string()),
        timestamp: header(headers, "ce-time"),
        spec_version: header(headers, "ce-specversion").or(Some("1.0".to_string())),
        data: body,
    })
}

fn header(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}
