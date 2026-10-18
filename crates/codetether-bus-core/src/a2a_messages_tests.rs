//! Regression coverage for task-scoped A2A publication from a borrowed bus.
use super::*;
use codetether_a2a_core::{MessageRole, Part};

#[test]
fn borrowed_bus_preserves_task_correlation_and_direction() {
    let bus = AgentBus::new();
    let mut receiver = bus.subscribe();
    let message = Message {
        message_id: "message-1".into(),
        role: MessageRole::User,
        parts: vec![Part::Text {
            text: "hello".into(),
        }],
        context_id: None,
        task_id: None,
        metadata: Default::default(),
        extensions: vec![],
    };
    for (publish, expected_from, expected_to) in [
        (
            inbound as fn(Option<&AgentBus>, &str, &str, &Message),
            "remote-a2a",
            "local",
        ),
        (outbound, "local", "remote-a2a"),
    ] {
        publish(Some(&bus), "local", "task-1", &message);
        let envelope = receiver.try_recv().unwrap();
        assert_eq!(envelope.topic, "task.task-1");
        assert_eq!(envelope.sender_id, "a2a");
        assert_eq!(envelope.correlation_id.as_deref(), Some("task-1"));
        match envelope.message {
            BusMessage::AgentMessage { from, to, parts } => {
                assert_eq!(from, expected_from);
                assert_eq!(to, expected_to);
                assert_eq!(
                    serde_json::to_value(parts).unwrap(),
                    serde_json::to_value(&message.parts).unwrap()
                );
            }
            other => panic!("unexpected message: {other:?}"),
        }
        assert!(receiver.try_recv().is_err());
        publish(None, "local", "task-1", &message);
    }
}
