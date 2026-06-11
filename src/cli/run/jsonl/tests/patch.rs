use super::{RunEvent, event_value};

#[test]
fn patch_approval_required_has_ids_and_timestamp() {
    let value = event_value(&RunEvent::PatchApprovalRequired {
        approval_id: "approval-1",
        patch_id: "patch-1",
        timestamp_ms: 371,
    });
    assert_eq!(value["type"], "run.patch_approval_required");
    assert_eq!(value["approval_id"], "approval-1");
    assert_eq!(value["patch_id"], "patch-1");
    assert_eq!(value["timestamp_ms"], 371);
}

#[test]
fn patch_completed_carries_metadata() {
    let metadata = serde_json::json!({"files": ["src/lib.rs"]});
    let value = event_value(&RunEvent::PatchCompleted {
        patch_id: "patch-1",
        timestamp_ms: 372,
        metadata: &metadata,
    });
    assert_eq!(value["type"], "run.patch_completed");
    assert_eq!(value["metadata"]["files"][0], "src/lib.rs");
}
