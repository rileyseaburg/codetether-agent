//! Environment configuration regression.

use super::*;

#[test]
fn test_is_configured() {
    let _lock = env_lock().lock().expect("env lock");
    // Always start clean - remove the var first to ensure isolation
    clean_env();

    let is_not_configured = !S3Sink::is_configured();
    assert!(
        is_not_configured,
        "S3 sink should not be configured by default"
    );

    unsafe {
        std::env::set_var("CODETETHER_S3_BUCKET", "test-bucket");
    }
    assert!(
        S3Sink::is_configured(),
        "S3 sink should be configured when bucket is set"
    );

    clean_env();
}
