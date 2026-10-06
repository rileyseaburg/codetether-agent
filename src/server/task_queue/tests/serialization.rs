//! Task receipt fields preserve older payloads and structured diagnostics.

use super::task;
use crate::server::task_queue::{KnativeTask, TaskCompletion};

#[test]
fn legacy_payload_without_completion_round_trips_without_new_null_fields() {
    let payload = serde_json::to_value(task("pending")).unwrap();
    for field in ["result", "error", "session_id", "diagnostics"] {
        assert!(payload.get(field).is_none(), "unexpected field: {field}");
    }
    let decoded: KnativeTask = serde_json::from_value(payload.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), payload);
}

#[test]
fn completion_round_trips_at_the_top_level_without_changing_metadata() {
    let mut completed = task("completed");
    completed.metadata = Some(serde_json::json!({"source": "caller"}));
    completed.completion = TaskCompletion {
        result: Some("output".into()),
        error: Some("detail".into()),
        session_id: Some("session-1".into()),
        diagnostics: Some(serde_json::json!({"attempt": 3, "metrics": [1, 2]})),
    };
    let payload = serde_json::to_value(&completed).unwrap();
    assert_eq!(payload["session_id"], "session-1");
    assert_eq!(payload["diagnostics"]["metrics"], serde_json::json!([1, 2]));
    assert_eq!(payload["metadata"]["source"], "caller");
    assert!(payload.get("completion").is_none());
    let decoded: KnativeTask = serde_json::from_value(payload.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), payload);
}
