//! Bounded task activity retained for A2A observers.

use crate::a2a::types::Task;
use dashmap::DashMap;
use serde_json::{Value, json};

const MAX_EVENTS: usize = 100;

pub(super) fn record(tasks: &DashMap<String, Task>, task_id: &str, event: &Value) {
    if matches!(
        event.get("type").and_then(Value::as_str),
        Some("ignored" | "tool_heartbeat" | "tool_output_chunk" | "usage_report")
    ) {
        return;
    }
    let Some(mut task) = tasks.get_mut(task_id) else {
        return;
    };
    let activity = task
        .metadata
        .entry("activity".to_string())
        .or_insert_with(|| Value::Array(Vec::new()));
    let Some(events) = activity.as_array_mut() else {
        return;
    };
    events.push(match event.get("type").and_then(Value::as_str) {
        Some("thinking_complete") => json!({ "type": "thinking_complete" }),
        _ => event.clone(),
    });
    if events.len() > MAX_EVENTS {
        events.drain(..events.len() - MAX_EVENTS);
    }
}

#[cfg(test)]
#[path = "server_task_activity_tests.rs"]
mod tests;
