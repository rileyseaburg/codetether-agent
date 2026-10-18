use super::fixtures::denied_metadata;
use crate::decline::DeclineGuard;
use serde_json::json;
use std::collections::HashMap;

#[test]
fn changed_or_non_denied_invocations_remain_available() {
    let mut guard = DeclineGuard::default();
    let first = json!({"cmd": "cargo publish"});
    guard.record("exec_command", &first, Some(&denied_metadata()));
    assert!(
        guard
            .blocked("exec_command", &json!({"cmd": "cargo test"}))
            .is_none()
    );
    guard.record("bash", &first, Some(&HashMap::new()));
    assert!(guard.blocked("bash", &first).is_none());
}
