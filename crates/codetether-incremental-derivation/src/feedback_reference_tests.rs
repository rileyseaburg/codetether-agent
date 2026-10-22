use codetether_provider_core::{ContentPart, Message, Role};
fn result(code: &str) -> Message {
    Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: "call".into(),
            content: format!(
                "Tool call feedback\n- status: failure\n\nOutput:\n{}",
                serde_json::json!({"error":{"code":code,"message":"Use the local server"}})
            ),
        }],
    }
}
#[test]
fn legacy_feedback_remains_labeled_tool_evidence_with_absolute_source() {
    let reference =
        super::feedback_reference::build(&[result("TOOL_APPROVAL_DENIED")], 16_000, 4183).unwrap();
    assert_eq!(reference.role, Role::Assistant);
    let ContentPart::Text { text } = &reference.content[0] else {
        panic!("text")
    };
    assert!(text.contains("tool evidence turn 4183"));
    assert!(text.contains("not independently authenticated user instructions"));
    assert!(text.contains("Use the local server"));
}
#[test]
fn ordinary_errors_and_tiny_budgets_do_not_create_feedback() {
    assert!(super::feedback_reference::build(&[result("NETWORK_ERROR")], 16_000, 0).is_none());
    assert!(super::feedback_reference::build(&[result("TOOL_APPROVAL_DENIED")], 1, 0).is_none());
}
