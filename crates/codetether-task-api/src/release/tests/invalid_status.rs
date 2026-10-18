//! invalid terminal status cannot mutate an active task.
//! Rejected releases must preserve state and must not notify observers.

use super::queue;

use codetether_task_queue::ReleaseError;

#[tokio::test]
async fn invalid_terminal_status_cannot_mutate_an_active_task() {
    let tasks = queue().await;
    assert!(matches!(
        tasks.release("task-1", "working").await,
        Err(ReleaseError::InvalidStatus)
    ));
    assert_eq!(tasks.get("task-1").await.unwrap().status, "working");
}
