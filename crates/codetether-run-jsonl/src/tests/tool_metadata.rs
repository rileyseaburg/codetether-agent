//! Tool metadata serialization regression.
use super::{RunEvent, event_value};

#[test]
fn tool_call_metadata_has_ids_timestamp_and_payload() {
    let metadata = serde_json::json!({"approval_request_id": "approval-1"});
    let value = event_value(&RunEvent::ToolCallMetadata {
        item_id: "item-3",
        tool_call_id: "call-3",
        timestamp_ms: 273,
        metadata: &metadata,
    });
    assert_eq!(value["type"], "run.tool_call_metadata");
    assert_eq!(value["item_id"], "item-3");
    assert_eq!(value["tool_call_id"], "call-3");
    assert_eq!(value["metadata"]["approval_request_id"], "approval-1");
}
