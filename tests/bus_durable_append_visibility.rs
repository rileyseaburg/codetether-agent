//! Append completion must make both the offset and replay visible immediately.

use chrono::Utc;
use codetether_agent::a2a::types::TaskState;
use codetether_agent::bus::durable_log::DurableLog;
use codetether_agent::bus::durable_log_file::FileDurableLog;
use codetether_agent::bus::{BusEnvelope, BusMessage};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn every_completed_append_has_a_visible_one_based_offset() {
    let directory = tempfile::tempdir().unwrap();
    let log = FileDurableLog::new(directory.path());
    for offset in 1..=128 {
        let envelope = BusEnvelope {
            id: format!("message-{offset}"),
            topic: "task.visibility".into(),
            sender_id: "visibility-test".into(),
            correlation_id: None,
            timestamp: Utc::now(),
            message: BusMessage::TaskUpdate {
                task_id: "visibility".into(),
                state: TaskState::Working,
                message: Some(format!("step {offset}")),
            },
        };
        assert_eq!(log.append(&envelope).await.unwrap(), offset);
        // No sleep or polling: append's return must establish visibility.
        let replay = log.tail("visibility", offset - 1).await.unwrap();
        assert_eq!(replay.len(), 1, "missing append at offset {offset}");
        assert_eq!(replay[0].id, envelope.id);
    }
    drop(log);
    let reopened = FileDurableLog::new(directory.path());
    assert_eq!(reopened.tail("visibility", 0).await.unwrap().len(), 128);
    assert!(reopened.tail("visibility", 128).await.unwrap().is_empty());
}
