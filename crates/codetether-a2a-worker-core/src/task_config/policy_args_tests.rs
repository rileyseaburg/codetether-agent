use super::from_metadata;
use serde_json::json;

#[test]
fn extracts_top_level_approval_id() {
    let metadata = json!({"approval_id": "approval-1"})
        .as_object()
        .cloned()
        .expect("object");
    assert_eq!(from_metadata(&metadata)["approval_id"], "approval-1");
}

#[test]
fn extracts_nested_forage_approval_id() {
    let metadata = json!({"forage": {"approval_id": "approval-2"}})
        .as_object()
        .cloned()
        .expect("object");
    assert_eq!(from_metadata(&metadata)["approval_id"], "approval-2");
}
