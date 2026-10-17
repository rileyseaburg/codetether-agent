//! Original test_complete_run test.
use super::*;

#[tokio::test]
async fn test_complete_run() {
    let store = InMemoryStore::new();
    store.create_run(&test_run_state()).await.unwrap();

    store
        .complete_run("test-run-1", RalphStatus::Completed)
        .await
        .unwrap();

    let run = store.get_run("test-run-1").await.unwrap().unwrap();
    assert_eq!(run.status, RalphStatus::Completed);
    assert!(run.completed_at.is_some());
}
