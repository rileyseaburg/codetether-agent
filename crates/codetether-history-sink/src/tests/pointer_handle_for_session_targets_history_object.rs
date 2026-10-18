use crate::{HistorySinkConfig, PointerHandle};

#[test]
fn pointer_handle_for_session_targets_history_object() {
    let cfg = HistorySinkConfig {
        endpoint: "http://x".to_string(),
        access_key: "a".to_string(),
        secret_key: "s".to_string(),
        bucket: "b".to_string(),
        prefix: "history/".to_string(),
    };
    let handle = PointerHandle::for_session(&cfg, "sess");
    assert_eq!(handle.bucket, "b");
    assert_eq!(handle.key, "history/sess/history.jsonl");
    assert!(handle.byte_range.is_none());
}
