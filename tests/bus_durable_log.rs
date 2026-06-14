//! Integration test: durable bus log persists coordination messages and
//! replays them for a late-joining/restarted consumer, while presence
//! messages are not persisted.

use chrono::Utc;
use codetether_agent::a2a::types::TaskState;
use codetether_agent::bus::durable_log::DurableLog;
use codetether_agent::bus::durable_log_file::FileDurableLog;
use codetether_agent::bus::{AgentBus, BusEnvelope, BusMessage};
use std::sync::Arc;
use uuid::Uuid;

fn envelope(topic: &str, message: BusMessage) -> BusEnvelope {
    BusEnvelope {
        id: Uuid::new_v4().to_string(),
        topic: topic.to_string(),
        sender_id: "tester".to_string(),
        correlation_id: None,
        timestamp: Utc::now(),
        message,
    }
}

#[tokio::test]
async fn durable_append_and_replay_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let log = FileDurableLog::new(dir.path());

    let env = envelope(
        "task.t1",
        BusMessage::TaskUpdate {
            task_id: "t1".into(),
            state: TaskState::Working,
            message: Some("step 1".into()),
        },
    );
    let off = log.append(&env).await.unwrap();
    assert_eq!(off, 1, "first append is offset 1");

    // Late joiner replays from the beginning.
    let replayed = log.tail("t1", 0).await.unwrap();
    assert_eq!(replayed.len(), 1);
    assert_eq!(replayed[0].topic, "task.t1");

    // Replay after the latest offset yields nothing.
    assert!(log.tail("t1", 1).await.unwrap().is_empty());
}

#[tokio::test]
async fn presence_messages_are_not_persisted() {
    let dir = tempfile::tempdir().unwrap();
    let log: Arc<dyn DurableLog> = Arc::new(FileDurableLog::new(dir.path()));
    let bus = AgentBus::new().with_durable_log(Arc::clone(&log)).into_arc();

    // Heartbeat is presence-only -> must NOT be appended.
    bus.publish(envelope(
        "agent.a1",
        BusMessage::Heartbeat {
            agent_id: "a1".into(),
            status: "ok".into(),
        },
    ));

    // Durable coordination message -> appended under its task partition.
    bus.publish(envelope(
        "task.t9",
        BusMessage::TaskUpdate {
            task_id: "t9".into(),
            state: TaskState::Completed,
            message: None,
        },
    ));

    // Give the fire-and-forget append task time to flush.
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    assert!(bus.replay("_global", 0).await.is_empty(), "no presence in global");
    let task = bus.replay("t9", 0).await;
    assert_eq!(task.len(), 1, "exactly one durable coordination message");
}
