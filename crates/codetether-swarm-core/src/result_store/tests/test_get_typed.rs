use super::*;

#[tokio::test]
async fn test_get_typed() {
    let store = ResultStore::new();

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        count: i32,
    }

    let data = TestData {
        name: "test".to_string(),
        count: 42,
    };

    store
        .publish("typed-key", "task-1", &data, vec![], None)
        .await
        .unwrap();

    let retrieved: TestData = store.get_typed("typed-key").await.unwrap();
    assert_eq!(retrieved, data);
}
