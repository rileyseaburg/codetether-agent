use crate::session::SessionEvent;

use super::mapper;

#[test]
fn bash_tool_start_emits_command_started() {
    let mut mapper = mapper();
    let events = mapper.map_session_event(&SessionEvent::ToolCallStart {
        tool_call_id: "call-1".into(),
        name: "bash".into(),
        arguments: r#"{"command":"echo ok","cwd":"/tmp"}"#.into(),
    });
    assert_eq!(events[0].kind, "tool.started");
    assert_eq!(events[1].kind, "command.started");
    assert_eq!(events[1].payload["command"], "echo ok");
}
