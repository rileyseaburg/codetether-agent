//! Existing configuration-default regression.
use super::*;

#[test]
fn test_config_defaults() {
    let config = BusS3SinkConfig {
        endpoint: "http://localhost:9000".to_string(),
        access_key: "key".to_string(),
        secret_key: "secret".to_string(),
        bucket: "test".to_string(),
        prefix: default_prefix(),
        batch_size: default_batch_size(),
        flush_interval_secs: default_flush_interval_secs(),
        secure: false,
        ignore_cert: false,
    };

    assert_eq!(config.prefix, "training/");
    assert_eq!(config.batch_size, 100);
    assert_eq!(config.flush_interval_secs, 30);
}
