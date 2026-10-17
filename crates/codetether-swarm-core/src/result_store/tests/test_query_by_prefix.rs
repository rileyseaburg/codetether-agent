use super::*;

#[tokio::test]
async fn test_query_by_prefix() {
    let store = ResultStore::new();

    store
        .publish("prefix/key-1", "task-1", "value-1", vec![], None)
        .await
        .unwrap();

    store
        .publish("prefix/key-2", "task-2", "value-2", vec![], None)
        .await
        .unwrap();

    store
        .publish("other/key-3", "task-1", "value-3", vec![], None)
        .await
        .unwrap();

    let results = store.query_by_prefix("prefix/").await;
    assert_eq!(results.len(), 2);
}
