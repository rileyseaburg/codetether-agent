use super::{RunEvent, event_value};

#[test]
fn command_events_include_command_ids_and_status() {
    let start = event_value(&RunEvent::CommandStarted {
        item_id: "item-1",
        tool_call_id: "call-1",
        timestamp_ms: 10,
        command: Some("echo ok"),
        cwd: Some("/tmp"),
    });
    assert_eq!(start["type"], "run.command_started");
    assert_eq!(start["item_id"], "item-1");
    assert_eq!(start["command"], "echo ok");
    let done = event_value(&RunEvent::CommandCompleted {
        item_id: "item-1",
        tool_call_id: "call-1",
        timestamp_ms: 11,
        success: true,
        duration_ms: 9,
    });
    assert_eq!(done["type"], "run.command_completed");
    assert_eq!(done["success"], true);
}
