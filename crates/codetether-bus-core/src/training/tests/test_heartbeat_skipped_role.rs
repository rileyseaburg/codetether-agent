//! Existing training conversion regression.
use super::*;

#[test]
fn test_heartbeat_skipped_role() {
    let env = BusEnvelope {
        id: "env-4".into(),
        topic: "broadcast".into(),
        sender_id: "agent-0".into(),
        correlation_id: None,
        timestamp: Utc::now(),
        message: BusMessage::Heartbeat {
            agent_id: "agent-0".into(),
            status: "ok".into(),
        },
    };

    let record = envelope_to_training_record(&env);
    // Heartbeats produce a record but flush_batch filters them out
    assert_eq!(record.role, "system");
    assert!(record.content.is_none());
}
