use super::SessionEvent;

use super::mapper;

#[test]
fn tool_lifecycle_reuses_tool_id() {
    let mut mapper = mapper();
    let start = mapper.map_session_event(&SessionEvent::ToolCallStart {
        tool_call_id: "call-1".into(),
        name: "bash".into(),
        arguments: "{\"cmd\":\"ls\"}".into(),
    });
    let done = mapper.map_session_event(&SessionEvent::ToolCallComplete {
        tool_call_id: "call-1".into(),
        name: "bash".into(),
        output: "ok".into(),
        success: true,
        duration_ms: 7,
    });
    assert_eq!(start[0].kind, "tool.started");
    assert_eq!(done[0].kind, "tool.completed");
    assert_eq!(
        start[0].payload["tool_call_id"],
        done[0].payload["tool_call_id"]
    );
    assert_eq!(done[0].payload["duration_ms"], 7);
}
