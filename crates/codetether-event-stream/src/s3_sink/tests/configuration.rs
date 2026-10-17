//! Environment configuration regression.

use super::*;

#[test]
fn test_config_from_env() {
    let _lock = env_lock().lock().expect("env lock");
    // Always start clean
    clean_env();

    unsafe {
        std::env::set_var("CODETETHER_S3_BUCKET", "test-bucket");
        std::env::set_var("CODETETHER_S3_PREFIX", "audit/");
        std::env::set_var(
            "CODETETHER_S3_ENDPOINT",
            "https://test.r2.cloudflarestorage.com",
        );
    }

    let config = S3SinkConfig::from_env();
    assert!(config.is_some());

    let cfg = config.unwrap();
    assert_eq!(cfg.bucket, "test-bucket");
    assert_eq!(cfg.prefix, "audit/");
    assert_eq!(
        cfg.endpoint,
        Some("https://test.r2.cloudflarestorage.com".to_string())
    );

    // Clean up
    clean_env();
}
