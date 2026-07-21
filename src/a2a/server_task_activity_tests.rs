use super::*;
use crate::a2a::types::{TaskState, TaskStatus};
use serde_json::json;
use std::collections::HashMap;

fn task() -> Task {
    Task {
        id: "task-1".into(),
        context_id: None,
        status: TaskStatus {
            state: TaskState::Working,
            message: None,
            timestamp: None,
        },
        artifacts: vec![],
        history: vec![],
        metadata: HashMap::new(),
    }
}

#[test]
fn retains_bounded_structured_activity() {
    let tasks = DashMap::new();
    tasks.insert("task-1".into(), task());
    for index in 0..105 {
        record(
            &tasks,
            "task-1",
            &json!({"type": "text_chunk", "text": index}),
        );
    }
    let task = tasks.get("task-1").unwrap();
    let events = task.metadata["activity"].as_array().unwrap();
    assert_eq!(events.len(), 100);
    assert_eq!(events[0]["text"], 5);
}

#[test]
fn drops_noisy_tool_stream_events() {
    let tasks = DashMap::new();
    tasks.insert("task-1".into(), task());
    record(&tasks, "task-1", &json!({"type": "tool_heartbeat"}));
    record(&tasks, "task-1", &json!({"type": "tool_output_chunk"}));
    let task = tasks.get("task-1").unwrap();
    assert!(task.metadata.get("activity").is_none());
}
