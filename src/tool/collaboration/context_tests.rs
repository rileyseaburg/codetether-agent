//! Delegation preserves exact receipts and signed per-session network state.

use serde_json::{Map, Value, json};

#[test]
fn inject_preserves_signed_network_denial_and_receipt() {
    let _lock = crate::approval::test_env::lock_env();
    let _network = crate::tool::network_access::test_env::Network::set("1");
    let mut input = json!({
        "__ct_session_id": "session-a",
        "__ct_parent_workspace": "/workspace",
        "approval_id": "approval-1",
    });
    crate::tool::network_access::bind_trusted(&mut input, false);
    let context: super::RuntimeContext = serde_json::from_value(input).expect("context");
    assert_eq!(context.resume_config().network_allowed, Some(false));
    let mut payload = Map::new();
    context.inject(&mut payload);
    let forwarded = Value::Object(payload);

    assert_eq!(forwarded["approval_id"], "approval-1");
    assert!(!crate::tool::network_access::allowed_for(&forwarded));
}