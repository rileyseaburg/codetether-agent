use super::fixture::event;
use crate::ThreadStore;

#[tokio::test]
async fn append_then_read_events() {
    let temp = tempfile::tempdir().unwrap();
    let store = ThreadStore::new(temp.path());
    let (first, second) = (event("thread-1", "event-1"), event("thread-1", "event-2"));

    store.append(first.clone()).await.unwrap();
    store.append(second.clone()).await.unwrap();

    let events = store.read_thread("thread-1").await.unwrap();
    assert_eq!(events, vec![first, second]);
    let raw = tokio::fs::read_to_string(temp.path().join("thread-1.jsonl"))
        .await
        .unwrap();
    assert_eq!(raw.lines().count(), 2);
}
