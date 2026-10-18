use crate::HistorySinkConfig;

#[test]
fn config_object_key_joins_prefix_and_session_id() {
    let cfg = HistorySinkConfig {
        endpoint: "http://x".to_string(),
        access_key: "a".to_string(),
        secret_key: "s".to_string(),
        bucket: "b".to_string(),
        prefix: "history/".to_string(),
    };
    assert_eq!(
        cfg.object_key("abc-123"),
        "history/abc-123/history.jsonl".to_string()
    );
}
