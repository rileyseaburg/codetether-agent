//! Apply legacy task field fallbacks in their original order.
use codetether_a2a_worker_core::cloudevents::CloudEvent;
pub(crate) fn extract(event: &CloudEvent) -> (String, String, String, String, i32) {
    let task_id = event
        .data
        .get("task_id")
        .or_else(|| event.data.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or(event.id.as_str())
        .to_string();
    let title = event
        .data
        .get("title")
        .or_else(|| event.data.get("prompt"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let description = event
        .data
        .get("description")
        .or_else(|| event.data.get("prompt"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let agent_type = event
        .data
        .get("agent_type")
        .or_else(|| event.data.get("agent"))
        .and_then(|v| v.as_str())
        .unwrap_or("build")
        .to_string();
    let priority = event
        .data
        .get("priority")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;

    (task_id, title, description, agent_type, priority)
}
