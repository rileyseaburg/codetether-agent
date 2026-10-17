//! Existing task-routing regression cases.
use super::check_task_scope;
use serde_json::json;

#[test]
fn accepts_empty_workspace_when_worker_has_none() {
    let task = json!({"id": "t1"});
    assert!(check_task_scope(&task, "w1", "builder", &[]).is_ok());
}

#[test]
fn nested_task_metadata_agent() {
    let task = json!({"task": {"id": "t1", "metadata": {"target_agent_name": "builder"}}});
    assert!(check_task_scope(&task, "w1", "builder", &[]).is_ok());
}

#[test]
fn nested_task_metadata_wrong_agent() {
    let task = json!({"task": {"id": "t1", "metadata": {"target_agent_name": "planner"}}});
    assert!(check_task_scope(&task, "w1", "builder", &[]).is_err());
}
