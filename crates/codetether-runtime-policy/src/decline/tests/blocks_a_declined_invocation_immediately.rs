use super::fixtures::denied_metadata;
use crate::decline::DeclineGuard;
use serde_json::json;

#[test]
fn blocks_a_declined_invocation_immediately() {
    let mut guard = DeclineGuard::default();
    let input = json!({"cmd": "cargo publish", "justification": "first reason"});
    guard.record("exec_command", &input, Some(&denied_metadata()));
    assert!(
        guard
            .blocked("read", &json!({"path": "Cargo.toml"}))
            .is_none()
    );
    let retry = json!({"cmd": "cargo publish", "justification": "new reason"});
    assert!(guard.blocked("exec_command", &retry).is_some());
}
