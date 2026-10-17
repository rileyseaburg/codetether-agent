//! Existing task-only schema regression test.
use serde_json::json;

#[test]
fn session_tasks_schema_advertises_only_work_item_operations() {
    let schema = super::value();
    assert_eq!(
        schema["properties"]["action"]["enum"],
        json!(["task_add", "task_status", "list"])
    );
    assert!(schema["properties"].get("objective").is_none());
}
