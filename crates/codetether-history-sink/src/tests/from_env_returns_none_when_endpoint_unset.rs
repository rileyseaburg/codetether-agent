use crate::HistorySinkConfig;

#[test]
fn from_env_returns_none_when_endpoint_unset() {
    // SAFETY: modifying env vars in tests is required for coverage
    // of the disabled-sink code path. `unsafe` is mandated by Rust
    // 2024's tightening of `remove_var`.
    unsafe {
        std::env::remove_var("CODETETHER_HISTORY_S3_ENDPOINT");
    }
    let cfg = HistorySinkConfig::from_env().unwrap();
    assert!(cfg.is_none());
}
