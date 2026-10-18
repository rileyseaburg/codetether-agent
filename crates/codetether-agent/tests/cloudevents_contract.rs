use axum::http::HeaderMap;
use codetether_agent::cloudevents::parse_cloud_event;
use serde_json::json;

#[test]
fn parses_structured_cloudevent() {
    let event = parse_cloud_event(
        &HeaderMap::new(),
        json!({"id":"evt-1","source":"codetether:a2a-server","type":"codetether.task.created","specversion":"1.0","data":{"task_id":"task-1"}}),
    )
    .expect("structured event should parse");
    assert_eq!(event.event_type, "codetether.task.created");
    assert_eq!(event.data["task_id"], "task-1");
}

#[test]
fn parses_binary_task_payload() {
    let mut headers = HeaderMap::new();
    headers.insert("ce-id", "evt-2".parse().unwrap());
    headers.insert("ce-source", "codetether:a2a-server".parse().unwrap());
    headers.insert("ce-type", "codetether.task.created".parse().unwrap());
    let event = parse_cloud_event(&headers, json!({"task_id":"task-2"}))
        .expect("binary event should parse");
    assert_eq!(event.id, "evt-2");
    assert_eq!(event.data["task_id"], "task-2");
}
