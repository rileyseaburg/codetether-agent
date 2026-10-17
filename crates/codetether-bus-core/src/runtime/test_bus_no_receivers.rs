//! Existing bus runtime regression coverage.
use super::*;
use chrono::Utc;

#[tokio::test]
async fn test_bus_no_receivers() {
    let bus = AgentBus::new().into_arc();
    // No handles created — send should succeed with 0 receivers
    let env = BusEnvelope {
        id: "test".into(),
        topic: "broadcast".into(),
        sender_id: "nobody".into(),
        correlation_id: None,
        timestamp: Utc::now(),
        message: BusMessage::Heartbeat {
            agent_id: "nobody".into(),
            status: "ok".into(),
        },
    };
    let count = bus.publish(env);
    assert_eq!(count, 0);
}
