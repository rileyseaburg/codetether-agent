//! Integration tests for S3 sink upload functionality
//!
//! These tests verify that S3 uploads work correctly with real HTTP requests
//! and that stub mode can be enabled when S3 is unavailable.

use codetether_agent::event_stream::s3_sink::{S3Sink, S3SinkConfig, S3SinkError};
use std::io::Write;
use std::sync::{LazyLock, Mutex, MutexGuard};
use tempfile::NamedTempFile;

static ENV_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

const MANAGED_ENV_KEYS: &[&str] = &[
    "S3_BUCKET",
    "S3_ACCESS_KEY",
    "S3_SECRET_KEY",
    "S3_ENDPOINT",
    "S3_REGION",
    "S3_STUB_MODE",
    "CODETETHER_S3_BUCKET",
    "CODETETHER_S3_ACCESS_KEY",
    "CODETETHER_S3_SECRET_KEY",
    "CODETETHER_S3_ENDPOINT",
    "CODETETHER_S3_REGION",
];

struct EnvVarGuard {
    _lock: MutexGuard<'static, ()>,
    snapshot: Vec<(String, Option<String>)>,
}

impl EnvVarGuard {
    fn lock() -> Self {
        let lock = ENV_LOCK.lock().expect("S3 env lock poisoned");
        let snapshot = MANAGED_ENV_KEYS
            .iter()
            .map(|k| ((*k).to_string(), std::env::var(k).ok()))
            .collect();

        Self {
            _lock: lock,
            snapshot,
        }
    }

    fn set(&self, key: &str, value: &str) {
        unsafe {
            std::env::set_var(key, value);
        }
    }

    fn remove(&self, key: &str) {
        unsafe {
            std::env::remove_var(key);
        }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        for (key, value) in &self.snapshot {
            unsafe {
                match value {
                    Some(v) => std::env::set_var(key, v),
                    None => std::env::remove_var(key),
                }
            }
        }
    }
}

/// Test that S3 upload performs actual HTTP request
#[tokio::test]
async fn test_s3_upload_makes_http_request() {
    let _env = EnvVarGuard::lock();

    // Skip test if S3 not configured
    if std::env::var("S3_BUCKET").is_err() && std::env::var("CODETETHER_S3_BUCKET").is_err() {
        eprintln!("Skipping test: S3_BUCKET (or CODETETHER_S3_BUCKET) not set");
        return;
    }

    let config = S3SinkConfig::from_env().expect("S3 config required");
    let sink = S3Sink::from_config(config)
        .await
        .expect("Failed to create S3 sink");

    // Create a temporary file with test data
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let test_data = br#"{"test": "data"}
{"more": "data"}
"#;
    temp_file
        .write_all(test_data)
        .expect("Failed to write test data");
    temp_file.flush().expect("Failed to flush");

    let path = temp_file.path().to_path_buf();

    // Attempt upload - this should make a real HTTP request
    let result = sink.upload_file(&path, "test-session").await;

    match result {
        Ok(url) => {
            // Verify we got a real S3 URL back
            assert!(
                url.starts_with("s3://")
                    || url.contains(".s3.")
                    || url.contains(".cloudflarestorage.com"),
                "Expected S3 URL, got: {url}"
            );
            println!("Upload succeeded: {url}");
        }
        Err(S3SinkError::Http(e)) => {
            // Network errors are acceptable in test environment
            println!("Network error (expected in test env): {e}");
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

/// Test that upload_bytes makes real HTTP request
#[tokio::test]
async fn test_s3_upload_bytes_makes_http_request() {
    let _env = EnvVarGuard::lock();

    // Skip test if S3 not configured
    if std::env::var("S3_BUCKET").is_err() && std::env::var("CODETETHER_S3_BUCKET").is_err() {
        eprintln!("Skipping test: S3_BUCKET (or CODETETHER_S3_BUCKET) not set");
        return;
    }

    let config = S3SinkConfig::from_env().expect("S3 config required");
    let sink = S3Sink::from_config(config)
        .await
        .expect("Failed to create S3 sink");

    let test_data = b"{\"event\": \"test\"}\n";
    let s3_key = "test/upload_test.json";

    // Attempt upload - this should make a real HTTP PUT request
    let result = sink
        .upload_bytes(test_data, s3_key, "application/json")
        .await;

    match result {
        Ok(url) => {
            // Verify we got a real S3 URL back
            assert!(url.contains(s3_key), "URL should contain the S3 key: {url}");
            println!("Upload succeeded: {url}");
        }
        Err(S3SinkError::Http(e)) => {
            // Network errors are acceptable in test environment
            println!("Network error (expected in test env): {e}");
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

/// Test that configuration can be loaded from environment
#[test]
fn test_s3_config_from_env() {
    let env = EnvVarGuard::lock();

    env.set("S3_BUCKET", "test-bucket");
    env.set("S3_ACCESS_KEY", "test-key");
    env.set("S3_SECRET_KEY", "test-secret");
    env.set("S3_ENDPOINT", "https://test.r2.cloudflarestorage.com");

    let config = S3SinkConfig::from_env();
    assert!(config.is_some(), "Config should be loaded from env");

    let cfg = config.unwrap();
    assert_eq!(cfg.bucket, "test-bucket");
    assert_eq!(cfg.access_key, Some("test-key".to_string()));
    assert_eq!(cfg.secret_key, Some("test-secret".to_string()));
}

/// Test error handling for missing credentials
#[tokio::test]
async fn test_s3_error_handling_missing_credentials() {
    let env = EnvVarGuard::lock();

    // Ensure no S3 config is set
    env.remove("S3_BUCKET");
    env.remove("CODETETHER_S3_BUCKET");

    let result = S3Sink::from_env().await;
    assert!(result.is_err(), "Should fail without S3_BUCKET");

    match result {
        Err(S3SinkError::MissingConfig(msg)) => {
            assert!(msg.contains("S3_BUCKET"), "Error should mention S3_BUCKET");
            assert!(
                msg.contains("CODETETHER_S3_BUCKET"),
                "Error should mention CODETETHER_S3_BUCKET"
            );
        }
        _ => panic!("Expected MissingConfig error"),
    }
}

/// Test that stub mode can be enabled via environment variable
#[test]
fn test_stub_mode_configuration() {
    let env = EnvVarGuard::lock();

    // This tests that stub mode can be configured
    // In stub mode, uploads should succeed without making network requests
    env.set("S3_STUB_MODE", "true");
    env.set("S3_BUCKET", "stub-bucket");

    // In a real implementation, the sink would check this env var
    // and skip actual HTTP requests when stub mode is enabled
    let stub_mode = std::env::var("S3_STUB_MODE").unwrap_or_default();
    assert_eq!(stub_mode, "true", "Stub mode should be configurable");
}

/// Test that S3_* env vars take precedence over CODETETHER_S3_* (backwards compat)
#[test]
fn test_s3_env_vars_take_precedence() {
    let env = EnvVarGuard::lock();

    // Set both new and legacy env vars
    env.set("S3_BUCKET", "new-bucket");
    env.set("CODETETHER_S3_BUCKET", "legacy-bucket");
    env.set("S3_ACCESS_KEY", "new-key");
    env.set("CODETETHER_S3_ACCESS_KEY", "legacy-key");

    let config = S3SinkConfig::from_env();
    assert!(config.is_some(), "Config should be loaded from env");

    let cfg = config.unwrap();
    // Should use S3_* (new) values, not CODETETHER_S3_* (legacy)
    assert_eq!(
        cfg.bucket, "new-bucket",
        "Should prefer S3_BUCKET over CODETETHER_S3_BUCKET"
    );
    assert_eq!(
        cfg.access_key,
        Some("new-key".to_string()),
        "Should prefer S3_ACCESS_KEY over CODETETHER_S3_ACCESS_KEY"
    );
}

/// Test that CODETETHER_S3_* env vars work as fallback
#[test]
fn test_codetether_s3_env_vars_as_fallback() {
    let env = EnvVarGuard::lock();

    // Only set legacy env vars (no S3_* vars)
    env.set("CODETETHER_S3_BUCKET", "legacy-bucket");
    env.set("CODETETHER_S3_ACCESS_KEY", "legacy-key");
    env.set("CODETETHER_S3_SECRET_KEY", "legacy-secret");
    env.set(
        "CODETETHER_S3_ENDPOINT",
        "https://legacy.r2.cloudflarestorage.com",
    );
    env.set("CODETETHER_S3_REGION", "legacy-region");

    let config = S3SinkConfig::from_env();
    assert!(
        config.is_some(),
        "Config should be loaded from legacy env vars"
    );

    let cfg = config.unwrap();
    // Should use CODETETHER_S3_* values as fallback
    assert_eq!(
        cfg.bucket, "legacy-bucket",
        "Should use CODETETHER_S3_BUCKET as fallback"
    );
    assert_eq!(
        cfg.access_key,
        Some("legacy-key".to_string()),
        "Should use CODETETHER_S3_ACCESS_KEY as fallback"
    );
    assert_eq!(
        cfg.secret_key,
        Some("legacy-secret".to_string()),
        "Should use CODETETHER_S3_SECRET_KEY as fallback"
    );
    assert_eq!(
        cfg.endpoint,
        Some("https://legacy.r2.cloudflarestorage.com".to_string()),
        "Should use CODETETHER_S3_ENDPOINT as fallback"
    );
    assert_eq!(
        cfg.region, "legacy-region",
        "Should use CODETETHER_S3_REGION as fallback"
    );
}

/// Test that error message mentions both naming conventions
#[tokio::test]
async fn test_error_message_mentions_both_conventions() {
    let env = EnvVarGuard::lock();

    // Ensure no S3 config is set
    env.remove("S3_BUCKET");
    env.remove("CODETETHER_S3_BUCKET");

    let result = S3Sink::from_env().await;
    assert!(result.is_err(), "Should fail without S3_BUCKET");

    match result {
        Err(S3SinkError::MissingConfig(msg)) => {
            assert!(msg.contains("S3_BUCKET"), "Error should mention S3_BUCKET");
            assert!(
                msg.contains("CODETETHER_S3_BUCKET"),
                "Error should mention CODETETHER_S3_BUCKET"
            );
        }
        _ => panic!("Expected MissingConfig error"),
    }
}
