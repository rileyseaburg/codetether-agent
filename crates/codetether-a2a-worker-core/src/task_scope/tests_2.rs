//! Existing task-routing regression cases.
use super::check_task_scope;
use serde_json::json;

#[test]
fn accepts_matching_worker() {
    let task = json!({"id": "t1", "metadata": {"target_worker_id": "w1"}});
    assert!(check_task_scope(&task, "w1", "builder", &[]).is_ok());
}

#[test]
fn rejects_unknown_workspace() {
    let task = json!({"id": "t1", "workspace_id": "ws-999"});
    let workspaces = vec!["ws-1".into(), "ws-2".into()];
    assert!(check_task_scope(&task, "w1", "builder", &workspaces).is_err());
}

#[test]
fn accepts_known_workspace() {
    let task = json!({"id": "t1", "workspace_id": "ws-1"});
    let workspaces = vec!["ws-1".into(), "ws-2".into()];
    assert!(check_task_scope(&task, "w1", "builder", &workspaces).is_ok());
}

#[test]
fn accepts_global_workspace() {
    let task = json!({"id": "t1", "workspace_id": "global"});
    let workspaces = vec!["ws-1".into()];
    assert!(check_task_scope(&task, "w1", "builder", &workspaces).is_ok());
}
