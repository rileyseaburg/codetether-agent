use std::sync::Mutex;

use serde_json::json;

use super::{ThreadEvent, ThreadStore};

static ENV_LOCK: Mutex<()> = Mutex::new(());

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

#[tokio::test]
async fn unsafe_thread_ids_are_rejected() {
    let temp = tempfile::tempdir().unwrap();
    let store = ThreadStore::new(temp.path());
    assert!(store.append(event("../escape", "event-1")).await.is_err());
    assert!(store.read_thread("../escape").await.is_err());
}

#[tokio::test]
async fn from_config_writes_under_threads_dir() {
    let temp = tempfile::tempdir().unwrap();
    let store = store_from_config(temp.path());
    store.append(event("cfg-thread", "event-1")).await.unwrap();
    assert!(temp.path().join("threads/cfg-thread.jsonl").exists());
}

fn event(thread_id: &str, event_id: &str) -> ThreadEvent {
    ThreadEvent {
        event_id: event_id.into(),
        thread_id: thread_id.into(),
        session_id: thread_id.into(),
        turn_id: "turn-1".into(),
        kind: "turn.started".into(),
        timestamp_ms: 1,
        payload: json!({ "ok": true }),
    }
}

fn store_from_config(root: &std::path::Path) -> ThreadStore {
    let _guard = ENV_LOCK.lock().unwrap();
    // SAFETY: this focused test serializes access with ENV_LOCK and restores
    // the variable before returning.
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", root) };
    let store = ThreadStore::from_config().unwrap();
    // SAFETY: paired with the guarded set_var above.
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    store
}
