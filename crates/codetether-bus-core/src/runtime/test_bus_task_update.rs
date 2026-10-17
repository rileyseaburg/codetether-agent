//! Existing bus runtime regression coverage.
use super::*;

#[tokio::test]
async fn test_bus_task_update() {
    let bus = AgentBus::new().into_arc();
    let handle = bus.handle("worker-1");

    let h2 = bus.handle("observer");
    // need mutable for recv
    let mut h2 = h2;

    handle.send_task_update("task-42", TaskState::Working, Some("processing".into()));

    let env = h2.recv().await.unwrap();
    assert_eq!(env.topic, "task.task-42");
    match &env.message {
        BusMessage::TaskUpdate { task_id, state, .. } => {
            assert_eq!(task_id, "task-42");
            assert_eq!(*state, TaskState::Working);
        }
        other => panic!("unexpected: {other:?}"),
    }
}
