//! Original test_create_and_get_run test.
use super::*;

#[tokio::test]
async fn test_create_and_get_run() {
    let store = InMemoryStore::new();
    let state = test_run_state();

    store.create_run(&state).await.unwrap();

    let fetched = store.get_run("test-run-1").await.unwrap();
    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert_eq!(fetched.run_id, "test-run-1");
    assert_eq!(fetched.status, RalphStatus::Pending);
}
