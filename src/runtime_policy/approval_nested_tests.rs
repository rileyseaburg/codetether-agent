use super::for_tool;
use serde_json::json;

#[test]
fn nested_business_metadata_changes_scope() {
    let before = json!({"calls": [{"tool": "write", "args": {"path": "a"}}]});
    let after = json!({
        "calls": [{
            "tool": "write",
            "args": {"path": "a", "approval_id": "nested"}
        }]
    });
    assert_ne!(
        for_tool("batch", &before).resource,
        for_tool("batch", &after).resource
    );
}

#[test]
fn confirmation_changes_mutation_scope() {
    let preview = json!({"path": "a", "old_string": "x", "new_string": "y"});
    let confirmed = json!({
        "path": "a", "old_string": "x", "new_string": "y", "confirm": true
    });
    assert_ne!(
        for_tool("confirm_edit", &preview).resource,
        for_tool("confirm_edit", &confirmed).resource
    );
}

#[test]
fn multi_confirmation_changes_mutation_scope() {
    let preview = json!({"edits": [{"path": "a", "old_string": "x", "new_string": "y"}]});
    let confirmed = json!({
        "edits": [{"path": "a", "old_string": "x", "new_string": "y"}],
        "confirm": true
    });
    assert_ne!(
        for_tool("confirm_multiedit", &preview).resource,
        for_tool("confirm_multiedit", &confirmed).resource
    );
}
