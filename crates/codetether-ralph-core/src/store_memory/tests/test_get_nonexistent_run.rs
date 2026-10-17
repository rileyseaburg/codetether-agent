//! Original test_get_nonexistent_run test.
use super::*;

#[tokio::test]
async fn test_get_nonexistent_run() {
    let store = InMemoryStore::new();
    let result = store.get_run("nonexistent").await.unwrap();
    assert!(result.is_none());
}
