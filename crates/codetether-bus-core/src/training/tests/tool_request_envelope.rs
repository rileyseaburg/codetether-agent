//! Shared envelope fixture for training regression tests.
use super::*;

pub(super) fn tool_request_envelope(
    envelope_id: &str,
    request_id: &str,
    correlation_id: Option<&str>,
) -> BusEnvelope {
    BusEnvelope {
        id: envelope_id.into(),
        topic: "tools.read_file".into(),
        sender_id: "agent-0".into(),
        correlation_id: correlation_id.map(str::to_string),
        timestamp: Utc::now(),
        message: BusMessage::ToolRequest {
            request_id: request_id.into(),
            agent_id: "agent-0".into(),
            tool_name: "read_file".into(),
            arguments: Value::Null,
            step: 1,
        },
    }
}
