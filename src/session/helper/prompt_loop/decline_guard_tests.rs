use super::decline_guard::DeclineGuard;
use serde_json::{Value, json};
use std::collections::HashMap;

fn denied_metadata() -> HashMap<String, Value> {
    HashMap::from([("error_code".into(), json!("TOOL_APPROVAL_DENIED"))])
}

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

#[test]
fn approval_metadata_cannot_bypass_a_decline() {
    let mut guard = DeclineGuard::default();
    let input = json!({"command": "rm artifact"});
    guard.record("bash", &input, Some(&denied_metadata()));
    let retry = json!({"command": "rm artifact", "approval_id": "new-id"});
    assert!(guard.blocked("bash", &retry).is_some());
}

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
