use super::{SessionEvent, mapper};

#[test]
fn tool_metadata_reuses_open_tool_id() {
    let mut mapper = mapper();
    let start = mapper.map_session_event(&SessionEvent::ToolCallStart {
        tool_call_id: "call-1".into(),
        name: "bash".into(),
        arguments: "{}".into(),
    });
    let metadata = mapper.map_session_event(&SessionEvent::ToolCallMetadata {
        tool_call_id: "call-1".into(),
        name: "bash".into(),
        metadata: serde_json::json!({"approval_request_id": "approval-1"}),
    });
    assert_eq!(metadata[0].kind, "tool.metadata");
    assert_eq!(
        start[0].payload["tool_call_id"],
        metadata[0].payload["tool_call_id"]
    );
    assert_eq!(
        metadata[0].payload["metadata"]["approval_request_id"],
        "approval-1"
    );
}
