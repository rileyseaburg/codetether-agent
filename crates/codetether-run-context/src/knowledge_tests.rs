//! Snapshot opt-in parsing regression.

use super::enabled_value;

#[test]
fn parses_enabled_values() {
    assert!(enabled_value("1"));
    assert!(enabled_value("true"));
    assert!(enabled_value("YES"));
    assert!(enabled_value("on"));
    assert!(!enabled_value("false"));
}
