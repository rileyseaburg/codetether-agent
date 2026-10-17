//! Heartbeat environment interpretation in a separate process.
#[test]
fn heartbeat_limits_and_fields() {
    if std::env::var_os("CODETETHER_TEST_HEARTBEAT").is_some() {
        let config = crate::CognitionHeartbeatConfig::from_env();
        assert!(!config.enabled);
        assert!(!config.include_thought_summary);
        assert_eq!(config.interval_secs, 5);
        assert_eq!(config.request_timeout_ms, 250);
        assert_eq!(config.source_base_url, "http://fixture");
        assert_eq!(config.provider_name, "fixture");
        assert_eq!(config.summary_max_chars, 12);
        return;
    }
    let result = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "consolidation::cognition::heartbeat_limits_and_fields",
        ])
        .env("CODETETHER_TEST_HEARTBEAT", "1")
        .env("CODETETHER_WORKER_COGNITION_SHARE_ENABLED", "false")
        .env("CODETETHER_WORKER_COGNITION_INCLUDE_THOUGHTS", "false")
        .env("CODETETHER_WORKER_COGNITION_INTERVAL_SECS", "1")
        .env("CODETETHER_WORKER_COGNITION_TIMEOUT_MS", "1")
        .env(
            "CODETETHER_WORKER_COGNITION_SOURCE_URL",
            "http://fixture///",
        )
        .env("CODETETHER_WORKER_COGNITION_PROVIDER", "fixture")
        .env("CODETETHER_WORKER_COGNITION_THOUGHT_MAX_CHARS", "12")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
