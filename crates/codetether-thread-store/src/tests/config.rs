use super::config_fixture::store_from_config;
use super::fixture::event;

#[tokio::test]
async fn from_config_writes_under_threads_dir() {
    let temp = tempfile::tempdir().unwrap();
    let store = store_from_config(temp.path());
    store.append(event("cfg-thread", "event-1")).await.unwrap();
    assert!(temp.path().join("threads/cfg-thread.jsonl").exists());
}
