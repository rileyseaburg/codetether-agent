//! Existing training conversion regression.
use super::*;

#[test]
fn test_training_record_tool_response() {
    let env = BusEnvelope {
        id: "env-2".into(),
        topic: "tools.read_file".into(),
        sender_id: "agent-0".into(),
        correlation_id: Some("corr-1".into()),
        timestamp: Utc::now(),
        message: BusMessage::ToolResponse {
            request_id: "req-1".into(),
            agent_id: "agent-0".into(),
            tool_name: "read_file".into(),
            result: "fn main() {}".into(),
            success: true,
            step: 1,
        },
    };

    let record = envelope_to_training_record(&env);
    assert_eq!(record.role, "tool");
    assert_eq!(record.content.as_deref(), Some("fn main() {}"));
    assert_eq!(record.tool_call_id.as_deref(), Some("req-1"));
    assert_eq!(record.name.as_deref(), Some("read_file"));
    assert_eq!(record.metadata.bus_kind, "tool_response");
}
