//! Existing bus runtime regression coverage.
use super::*;

#[tokio::test]
async fn test_bus_send_recv() {
    let bus = AgentBus::new().into_arc();
    let mut handle_a = bus.handle("agent-a");
    let mut handle_b = bus.handle("agent-b");

    handle_a.send_to_agent(
        "agent-b",
        vec![Part::Text {
            text: "hello".into(),
        }],
    );

    // Both handles receive the broadcast
    let env = handle_b.recv().await.unwrap();
    assert_eq!(env.topic, "agent.agent-b");
    match &env.message {
        BusMessage::AgentMessage { from, to, .. } => {
            assert_eq!(from, "agent-a");
            assert_eq!(to, "agent-b");
        }
        other => panic!("unexpected message: {other:?}"),
    }

    // handle_a also receives it (broadcast semantics)
    let env_a = handle_a.try_recv().unwrap();
    assert_eq!(env_a.topic, "agent.agent-b");
}
