use super::*;
use crate::a2a::types::{Message, MessageRole, Part};

#[test]
fn runtime_delivers_to_addressed_mailbox() {
    let runtime = ActorRuntime::new();
    runtime.register("docs-agent");

    let msg = ActorMessage::new(
        "indeed-agent",
        "docs-agent",
        "resume.requested",
        serde_json::json!({"title": "Rust Engineer"}),
    );
    runtime.send(msg.into_envelope());

    let received = runtime.receive(&ActorId::from("docs-agent")).unwrap();
    assert_eq!(received.from, ActorId::from("indeed-agent"));
    assert_eq!(received.kind, "resume.requested");
    assert!(runtime.ack(&received.id));
}

#[test]
fn local_a2a_message_round_trips_through_actor_envelope() {
    let message = Message {
        message_id: "m1".to_string(),
        role: MessageRole::User,
        parts: vec![Part::Text {
            text: "hello".into(),
        }],
        context_id: Some("ctx".to_string()),
        task_id: Some("task".to_string()),
        metadata: Default::default(),
        extensions: vec![],
    };

    let envelope = a2a::message_to_actor("agent-a", "agent-b", &message);
    let round_trip = a2a::actor_to_message(&envelope);

    assert_eq!(envelope.kind, "a2a.message");
    assert_eq!(round_trip.message_id, "m1");
}
