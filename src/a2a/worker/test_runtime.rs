use std::{collections::HashSet, sync::Arc};

use serde_json::json;
use tokio::sync::Mutex;

#[test]
fn task_timeout_prefers_fire_and_forget_payload_timeout() {
    let task =
        json!({"id": "task-1", "task_timeout_seconds": 604800, "metadata": {"timeout_secs": 1200}});
    let metadata = super::task_metadata(&task);
    assert_eq!(super::task_timeout_secs(&task, &metadata), 604800);
}

#[tokio::test]
async fn reserve_task_slot_enforces_capacity() {
    let processing = Arc::new(Mutex::new(HashSet::new()));
    assert_eq!(
        super::reserve_task_slot(&processing, "task-1", 2).await,
        super::TaskReservation::Reserved
    );
    assert_eq!(
        super::reserve_task_slot(&processing, "task-1", 2).await,
        super::TaskReservation::AlreadyProcessing
    );
    assert_eq!(
        super::reserve_task_slot(&processing, "task-2", 2).await,
        super::TaskReservation::Reserved
    );
    assert_eq!(
        super::reserve_task_slot(&processing, "task-3", 2).await,
        super::TaskReservation::AtCapacity
    );
}

#[test]
fn normalize_max_concurrent_tasks_never_returns_zero() {
    assert_eq!(super::normalize_max_concurrent_tasks(0), 1);
    assert_eq!(super::normalize_max_concurrent_tasks(3), 3);
}
