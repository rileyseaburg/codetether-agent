//! Text-only Vertex conversion retains roles, placeholders, and orphan results.
use super::super::fixtures::*;
use crate::provider::Role;
use serde_json::json;

#[test]
fn text_only_conversion_remains_unchanged() {
    let out = convert(&[
        message(Role::User, vec![text("hello")]),
        message(Role::User, vec![]),
        message(Role::Assistant, vec![text("reply")]),
        message(Role::Assistant, vec![]),
        message(Role::Tool, vec![result("orphan", "Error: retained")]),
    ]);
    assert_eq!(
        out["messages"],
        json!([
            {"role": "user", "content": [{"type": "text", "text": "hello"}]},
            {"role": "user", "content": [{"type": "text", "text": " "}]},
            {"role": "assistant", "content": [{"type": "text", "text": "reply"}]},
            {"role": "assistant", "content": [{"type": "text", "text": " "}]},
            {"role": "user", "content": [{"type": "tool_result", "tool_use_id": "orphan", "content": "Error: retained"}]},
        ])
    );
}
