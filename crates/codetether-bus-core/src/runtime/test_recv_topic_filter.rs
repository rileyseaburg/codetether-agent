//! Existing bus runtime regression coverage.
use super::*;

#[tokio::test]
async fn test_recv_topic_filter() {
    let bus = AgentBus::new().into_arc();
    let handle = bus.handle("agent-x");
    let mut listener = bus.handle("listener");

    // Send to two different topics
    handle.send(
        "task.1",
        BusMessage::TaskUpdate {
            task_id: "1".into(),
            state: TaskState::Working,
            message: None,
        },
    );
    handle.send(
        "task.2",
        BusMessage::TaskUpdate {
            task_id: "2".into(),
            state: TaskState::Completed,
            message: None,
        },
    );

    // recv_topic("task.2") should skip the first and return the second
    let env = listener.recv_topic("task.2").await.unwrap();
    match &env.message {
        BusMessage::TaskUpdate { task_id, .. } => assert_eq!(task_id, "2"),
        other => panic!("unexpected: {other:?}"),
    }
}
