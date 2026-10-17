//! Existing training conversion regression.
use super::*;

#[test]
fn test_training_record_agent_message() {
    let env = BusEnvelope {
        id: "env-3".into(),
        topic: "agent.planner".into(),
        sender_id: "coder".into(),
        correlation_id: None,
        timestamp: Utc::now(),
        message: BusMessage::AgentMessage {
            from: "coder".into(),
            to: "planner".into(),
            parts: vec![codetether_a2a_core::Part::Text {
                text: "I fixed the bug".into(),
            }],
        },
    };

    let record = envelope_to_training_record(&env);
    assert_eq!(record.role, "assistant");
    assert!(
        record
            .content
            .as_deref()
            .unwrap()
            .contains("I fixed the bug")
    );
    assert!(
        record
            .content
            .as_deref()
            .unwrap()
            .contains("[coder → planner]")
    );
}
