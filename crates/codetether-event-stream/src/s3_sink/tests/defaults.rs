//! Environment configuration regression.

use super::*;

#[test]
fn test_config_defaults() {
    let _lock = env_lock().lock().expect("env lock");
    clean_env();

    unsafe {
        std::env::set_var("CODETETHER_S3_BUCKET", "test-bucket");
    }

    let config = S3SinkConfig::from_env().unwrap();
    assert_eq!(config.region, "us-east-1");

    clean_env();
}
