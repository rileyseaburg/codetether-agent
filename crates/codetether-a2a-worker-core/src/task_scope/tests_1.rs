//! Existing task-routing regression cases.
use super::check_task_scope;
use serde_json::json;

#[test]
fn accepts_unscoped_task() {
    let task = json!({"id": "t1", "prompt": "hello"});
    assert!(check_task_scope(&task, "w1", "builder", &[]).is_ok());
}

#[test]
fn rejects_wrong_agent() {
    let task = json!({"id": "t1", "metadata": {"target_agent_name": "planner"}});
    assert!(check_task_scope(&task, "w1", "builder", &[]).is_err());
}

#[test]
fn accepts_matching_agent() {
    let task = json!({"id": "t1", "metadata": {"target_agent_name": "builder"}});
    assert!(check_task_scope(&task, "w1", "builder", &[]).is_ok());
}

#[test]
fn rejects_wrong_worker() {
    let task = json!({"id": "t1", "metadata": {"target_worker_id": "other-worker"}});
    assert!(check_task_scope(&task, "w1", "builder", &[]).is_err());
}
