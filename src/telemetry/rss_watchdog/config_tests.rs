//! Default RSS policy regression tests.

use super::config::Config;
use std::time::Duration;

#[test]
fn defaults_reclaim_transient_heap_before_one_gibibyte() {
    let config = Config::default();
    assert_eq!(config.warn_mib, 256);
    assert_eq!(config.critical_mib, 1024);
    assert_eq!(config.sample, Duration::from_secs(2));
    assert_eq!(config.trim, Duration::from_secs(15));
}

#[test]
fn warning_threshold_remains_below_critical() {
    let config = Config::default();
    assert!(config.warn_mib < config.critical_mib);
}
