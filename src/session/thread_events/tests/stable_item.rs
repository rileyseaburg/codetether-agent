use crate::session::SessionEvent;

use super::mapper;

#[test]
fn tool_and_command_events_share_item_id() {
    let mut mapper = mapper();
    let start = mapper.map_session_event(&SessionEvent::ToolCallStart {
        tool_call_id: "call-1".into(),
        name: "bash".into(),
        arguments: r#"{"command":"echo ok"}"#.into(),
    });
    let done = mapper.map_session_event(&SessionEvent::ToolCallComplete {
        tool_call_id: "call-1".into(),
        name: "bash".into(),
        output: "ok".into(),
        success: true,
        duration_ms: 3,
    });
    assert_eq!(start[0].payload["item_id"], start[1].payload["item_id"]);
    assert_eq!(start[0].payload["item_id"], done[0].payload["item_id"]);
    assert_eq!(done[0].payload["item_id"], done[1].payload["item_id"]);
    assert_ne!(done[0].payload["item_id"], "turn-1");
}
