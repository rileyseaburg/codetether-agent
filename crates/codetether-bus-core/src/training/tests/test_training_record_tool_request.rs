//! Existing training conversion regression.
use super::*;

#[test]
fn test_training_record_tool_request() {
    let env = BusEnvelope {
        id: "env-1".into(),
        topic: "tools.read_file".into(),
        sender_id: "agent-0".into(),
        correlation_id: Some("corr-1".into()),
        timestamp: Utc::now(),
        message: BusMessage::ToolRequest {
            request_id: "req-1".into(),
            agent_id: "agent-0".into(),
            tool_name: "read_file".into(),
            arguments: serde_json::json!({"path": "/src/main.rs"}),
            step: 1,
        },
    };

    let record = envelope_to_training_record(&env);
    assert_eq!(record.role, "assistant");
    assert!(record.content.is_none());
    let calls = record.tool_calls.unwrap();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].function.name, "read_file");
    assert_eq!(calls[0].call_type, "function");
    assert_eq!(record.metadata.bus_kind, "tool_request");
}
