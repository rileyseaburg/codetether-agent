use crate::tool::ToolResult;
use crate::tool::agent::event_loop::live_trace::LiveTraceEntry;

#[test]
fn remote_activity_projects_into_the_live_tui_trace() {
    let suffix = uuid::Uuid::new_v4();
    let name = format!("active-peer-{suffix}");
    let parent = format!("active-parent-{suffix}");
    let turn = super::begin(&name, Some(&parent), "inspect the workspace");
    super::record_activity(
        &name,
        Some(&parent),
        &[
            serde_json::json!({"type":"thinking"}),
            serde_json::json!({"type":"tool_call_start","name":"read","arguments":{"path":"README.md"}}),
            serde_json::json!({"type":"text_chunk","text":"Found the cause"}),
        ],
    );

    let trace = super::live_trace(&name, &parent).unwrap();
    assert_eq!(trace.prompt, "inspect the workspace");
    assert_eq!(trace.streaming_text.as_deref(), Some("Found the cause"));
    assert!(matches!(
        trace.entries.first(),
        Some(LiveTraceEntry::ToolCall { name, .. }) if name == "read"
    ));
    turn.complete(&ToolResult::success("done"));
}
