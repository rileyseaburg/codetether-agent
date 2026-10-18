use super::fixture::event;
use crate::ThreadStore;

#[tokio::test]
async fn unsafe_thread_ids_are_rejected() {
    let temp = tempfile::tempdir().unwrap();
    let store = ThreadStore::new(temp.path());
    assert!(store.append(event("../escape", "event-1")).await.is_err());
    assert!(store.read_thread("../escape").await.is_err());
}
