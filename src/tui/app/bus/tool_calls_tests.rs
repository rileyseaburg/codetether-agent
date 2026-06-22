//! Tests for [`ToolCallTracker`](super::tool_calls::ToolCallTracker).

use super::tool_calls::ToolCallTracker;
use crate::bus::BusMessage;
use std::time::Duration;

fn request(request_id: &str, agent: &str, tool: &str) -> BusMessage {
    BusMessage::ToolRequest {
        request_id: request_id.to_string(),
        agent_id: agent.to_string(),
        tool_name: tool.to_string(),
        arguments: serde_json::Value::Null,
        step: 1,
    }
}

fn response(request_id: &str) -> BusMessage {
    BusMessage::ToolResponse {
        request_id: request_id.to_string(),
        agent_id: "a".to_string(),
        tool_name: "bash".to_string(),
        result: String::new(),
        success: true,
        step: 1,
    }
}

#[test]
fn open_call_clears_on_response() {
    let mut t = ToolCallTracker::default();
    t.observe(&request("r1", "agent-a", "bash"));
    assert!(t.stalled(Duration::ZERO).is_some());
    t.observe(&response("r1"));
    assert!(t.stalled(Duration::ZERO).is_none());
}

#[test]
fn stalled_reports_agent_and_tool() {
    let mut t = ToolCallTracker::default();
    t.observe(&request("r1", "agent-a", "edit"));
    let call = t.stalled(Duration::ZERO).expect("open call");
    assert_eq!(call.agent_id, "agent-a");
    assert_eq!(call.tool_name, "edit");
}

#[test]
fn long_timeout_suppresses_alert() {
    let mut t = ToolCallTracker::default();
    t.observe(&request("r1", "agent-a", "bash"));
    assert!(t.stalled(Duration::from_secs(3600)).is_none());
}
