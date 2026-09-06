//! Preserve Vertex's special system encoding, including non-image filtering.
use super::super::fixtures::*;
use crate::provider::{ContentPart, Role};
use serde_json::json;

#[test]
fn system_and_developer_shapes_remain_vertex_specific() {
    let out = convert(&[message(
        Role::System,
        vec![text("system"), image("data:image/png;base64,YQ==")],
    )]);
    assert_eq!(out["system"], "system");
    assert_eq!(out["messages"], json!([]));
    let out = convert(&[
        message(Role::System, vec![text("system")]),
        message(Role::Developer, vec![text("developer")]),
    ]);
    assert_eq!(
        out["system"],
        json!([
            {"type": "text", "text": "system"}, {"type": "text", "text": "developer"}
        ])
    );
}
#[test]
fn lone_system_thinking_stays_omitted_and_signatures_stay_omitted() {
    let thinking = || ContentPart::Thinking {
        text: "reason".into(),
        signature: Some("sig".into()),
    };
    let out = convert(&[
        message(Role::System, vec![thinking()]),
        message(Role::Assistant, vec![thinking()]),
    ]);
    assert!(out["system"].is_null());
    assert_eq!(
        out["messages"][0]["content"],
        json!([
            {"type": "thinking", "thinking": "reason"}
        ])
    );
}
