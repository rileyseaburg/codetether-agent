//! Regression tests for chronological, non-duplicated training records.

use super::*;
use crate::bus::BusMessage;
use chrono::{TimeZone, Utc};

fn envelope(second: u32, message: BusMessage) -> BusEnvelope {
    BusEnvelope {
        id: format!("env-{second}"),
        topic: "agent.build.tool".into(),
        sender_id: "build".into(),
        correlation_id: Some("turn-1".into()),
        timestamp: Utc.with_ymd_and_hms(2026, 7, 17, 20, 7, second).unwrap(),
        message,
    }
}

#[test]
fn drops_full_output_and_preserves_event_order() {
    let request = BusMessage::ToolRequest {
        request_id: "call-1".into(),
        agent_id: "build".into(),
        tool_name: "exec_command".into(),
        arguments: serde_json::json!({"cmd": "true"}),
        step: 1,
    };
    let response = BusMessage::ToolResponse {
        request_id: "call-1".into(),
        agent_id: "build".into(),
        tool_name: "exec_command".into(),
        result: "ok".into(),
        success: true,
        step: 1,
    };
    let full = BusMessage::ToolOutputFull {
        agent_id: "build".into(),
        tool_name: "exec_command".into(),
        output: "ok".into(),
        success: true,
        step: 1,
    };
    let incomplete = collect(&[envelope(0, request.clone())]);
    let records = collect(&[
        envelope(1, request),
        envelope(2, response),
        envelope(3, full),
    ]);
    assert!(incomplete.is_empty());
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].metadata.bus_kind, "tool_request_batch");
    assert_eq!(records[1].metadata.bus_kind, "tool_response");
}
