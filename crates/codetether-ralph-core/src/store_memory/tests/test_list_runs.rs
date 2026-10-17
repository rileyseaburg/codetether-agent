//! Original test_list_runs test.
use super::*;

#[tokio::test]
async fn test_list_runs() {
    let store = InMemoryStore::new();
    store.create_run(&test_run_state()).await.unwrap();

    let summaries = store.list_runs().await.unwrap();
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].run_id, "test-run-1");
    assert_eq!(summaries[0].total, 1);
}
