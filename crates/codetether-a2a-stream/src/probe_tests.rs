//! Regression cases for the parent stream module.

use super::probe_target;

#[test]
fn parses_targets_with_default_ports() {
    assert_eq!(probe_target("http://host/path").as_deref(), Some("host:80"));
    assert_eq!(probe_target("https://host").as_deref(), Some("host:443"));
    assert_eq!(
        probe_target("http://host:8001/x").as_deref(),
        Some("host:8001")
    );
    assert!(probe_target("ftp://host").is_none());
}
