use super::*;

#[tokio::test]
async fn test_query_by_tags() {
    let store = ResultStore::new();

    store
        .publish(
            "key-1",
            "task-1",
            "value-1",
            vec!["tag-a".to_string()],
            None,
        )
        .await
        .unwrap();

    store
        .publish(
            "key-2",
            "task-2",
            "value-2",
            vec!["tag-b".to_string()],
            None,
        )
        .await
        .unwrap();

    store
        .publish(
            "key-3",
            "task-1",
            "value-3",
            vec!["tag-a".to_string(), "tag-c".to_string()],
            None,
        )
        .await
        .unwrap();

    let results = store.query_by_tags(&["tag-a".to_string()]).await;
    assert_eq!(results.len(), 2);
}
