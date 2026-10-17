use super::allowed;
use serde_json::json;

#[test]
fn read_actions_are_allowed() {
    assert!(allowed("memory", &json!({"action": "search"})));
    assert!(allowed("okr", &json!({"action": "list_okrs"})));
    assert!(allowed("task", &json!({"action": " status "})));
}

#[test]
fn session_bookkeeping_is_allowed_for_every_action() {
    assert!(allowed("session_task", &json!({"action": "list"})));
    assert!(allowed("session_task", &json!({"action": "task_add"})));
}

#[test]
fn mutating_or_missing_actions_still_gate() {
    assert!(!allowed("memory", &json!({"action": "delete"})));
    assert!(!allowed("okr", &json!({"action": "delete_okr"})));
    assert!(!allowed("go", &json!({"action": "execute"})));
    assert!(!allowed("memory", &json!({})));
    assert!(!allowed("bash", &json!({"action": "list"})));
}
