//! Existing relay regression coverage.
use super::*;
use tokio::time::{Duration, timeout};

#[tokio::test]
async fn test_send_handoff_emits_agent_and_relay_topics() {
    let bus = AgentBus::new().into_arc();
    let relay = ProtocolRelayRuntime::with_relay_id(bus.clone(), "relay-test");
    let mut observer = bus.handle("observer");

    let correlation = relay.send_handoff("auto-planner", "auto-coder", "handoff payload");
    assert!(correlation.starts_with("relay-test:"));

    let first = timeout(Duration::from_millis(200), observer.recv())
        .await
        .expect("first envelope timeout")
        .expect("first envelope missing");
    let second = timeout(Duration::from_millis(200), observer.recv())
        .await
        .expect("second envelope timeout")
        .expect("second envelope missing");

    let topics = [first.topic, second.topic];
    assert!(topics.iter().any(|t| t == "agent.auto-coder"));
    assert!(topics.iter().any(|t| t == "relay.relay-test"));
}
