use super::mapper;

#[test]
fn patch_approval_event_carries_patch_ids() {
    let event = mapper().patch_approval_required("approval-1", "patch-1");
    assert_eq!(event.kind, "patch.approval_required");
    assert_eq!(event.payload["approval_id"], "approval-1");
    assert_eq!(event.payload["patch_id"], "patch-1");
}

#[test]
fn patch_metadata_emits_completion_and_approval_events() {
    let events = mapper().map_session_event(&crate::session::SessionEvent::ToolCallMetadata {
        tool_call_id: "patch-1".into(),
        name: "apply_patch".into(),
        metadata: serde_json::json!({
            "patch_files": ["src/lib.rs"],
            "patch_hunks": 1,
            "approval_request_id": "approval-1"
        }),
    });
    assert_eq!(events[1].kind, "patch.completed");
    assert_eq!(events[2].kind, "patch.approval_required");
    assert_eq!(events[2].payload["approval_id"], "approval-1");
}
