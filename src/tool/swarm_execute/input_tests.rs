use super::input::parse_tasks;
use serde_json::json;

#[test]
fn accepts_plain_task_strings() {
    let tasks = parse_tasks(&json!({"tasks": ["Inspect code", "Run tests"]})).unwrap();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].name, "Task 1");
    assert_eq!(tasks[0].instruction, "Inspect code");
}

#[test]
fn object_task_name_is_optional() {
    let tasks = parse_tasks(&json!({
        "tasks": [{"instruction": "Review code", "specialty": "Reviewer"}]
    }))
    .unwrap();
    assert_eq!(tasks[0].name, "Task 1");
    assert_eq!(tasks[0].specialty.as_deref(), Some("Reviewer"));
}

#[test]
fn preserves_named_object_tasks() {
    let tasks = parse_tasks(&json!({
        "tasks": [{"id": "review", "name": "Review", "instruction": "Review code"}]
    }))
    .unwrap();
    assert_eq!(tasks[0].id.as_deref(), Some("review"));
    assert_eq!(tasks[0].name, "Review");
}

#[test]
fn rejects_empty_tasks() {
    assert!(parse_tasks(&json!({"tasks": []})).is_err());
}
