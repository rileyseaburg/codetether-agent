//! Original test_update_status test.
use super::*;

#[tokio::test]
async fn test_update_status() {
    let store = InMemoryStore::new();
    store.create_run(&test_run_state()).await.unwrap();

    store
        .update_status("test-run-1", RalphStatus::Running)
        .await
        .unwrap();

    let run = store.get_run("test-run-1").await.unwrap().unwrap();
    assert_eq!(run.status, RalphStatus::Running);
    assert!(run.started_at.is_some());
}
