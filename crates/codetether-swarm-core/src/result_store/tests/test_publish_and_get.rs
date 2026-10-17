use super::*;

#[tokio::test]
async fn test_publish_and_get() {
    let store = ResultStore::new();

    store
        .publish("test-key", "task-1", "hello world", vec![], None)
        .await
        .unwrap();

    let result = store.get("test-key").await.unwrap();
    assert_eq!(result.key, "test-key");
    assert_eq!(result.producer_id, "task-1");
    assert_eq!(result.value, Value::String("hello world".to_string()));
}
