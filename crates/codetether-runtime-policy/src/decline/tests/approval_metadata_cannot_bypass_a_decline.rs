use super::fixtures::denied_metadata;
use crate::decline::DeclineGuard;
use serde_json::json;

#[test]
fn approval_metadata_cannot_bypass_a_decline() {
    let mut guard = DeclineGuard::default();
    let input = json!({"command": "rm artifact"});
    guard.record("bash", &input, Some(&denied_metadata()));
    let retry = json!({"command": "rm artifact", "approval_id": "new-id"});
    assert!(guard.blocked("bash", &retry).is_some());
}
