use super::user;
use crate::session::helper::runtime::block_prior_context;
use serde_json::json;

fn denied() -> Vec<crate::provider::Message> {
    vec![user("do not inspect session history")]
}

#[test]
fn blocks_unenforced_delegation_but_keeps_status_actions() {
    for (tool, action) in [
        ("go", "execute"),
        ("ralph", "run"),
        ("relay_autochat", "delegate"),
        ("relay_autochat", "handoff"),
    ] {
        assert!(block_prior_context(&denied(), tool, &json!({"action": action})).is_some());
    }
    assert!(block_prior_context(&denied(), "go", &json!({"action": "status"})).is_none());
    let batch = json!({"calls": [{"tool": "agent", "args": {"action": "spawn"}}]});
    assert!(block_prior_context(&denied(), "batch", &batch).is_some());
}
