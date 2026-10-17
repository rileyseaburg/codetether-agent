use super::*;

#[tokio::test]
async fn test_subscription_notifications() {
    let store = ResultStore::new();
    let mut rx = store.subscribe();

    store
        .publish(
            "notify-key",
            "task-1",
            "value",
            vec!["tag-1".to_string()],
            None,
        )
        .await
        .unwrap();

    let notification = rx.try_recv().unwrap();
    assert_eq!(notification.key, "notify-key");
    assert_eq!(notification.producer_id, "task-1");
}
