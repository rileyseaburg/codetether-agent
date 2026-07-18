use super::{ThreadStatus, get, initialize, remove, set, subscribe};

#[test]
fn publishes_status_changes_to_subscribers() {
    let id = "thread-status-publishes";
    initialize(id);
    let receiver = subscribe(id).expect("status receiver");
    set(id, ThreadStatus::Running);
    assert_eq!(*receiver.borrow(), ThreadStatus::Running);
    remove(id);
    assert_eq!(*receiver.borrow(), ThreadStatus::NotFound);
    assert_eq!(get(id), ThreadStatus::NotFound);
}

#[test]
fn interrupted_is_not_a_final_status() {
    assert!(!ThreadStatus::Interrupted.is_final());
    assert!(ThreadStatus::Completed(None).is_final());
}

#[test]
fn serializes_like_the_codex_protocol() {
    assert_eq!(
        serde_json::to_value(ThreadStatus::Running).unwrap(),
        "running"
    );
    assert_eq!(
        serde_json::to_value(ThreadStatus::Completed(Some("done".into()))).unwrap(),
        serde_json::json!({"completed":"done"})
    );
}

#[test]
fn interruption_is_not_overwritten_by_task_cancellation() {
    let id = "thread-status-interrupted";
    initialize(id);
    set(id, ThreadStatus::Interrupted);
    super::turn(id, "", Some("task cancelled"));
    assert_eq!(get(id), ThreadStatus::Interrupted);
    remove(id);
}
