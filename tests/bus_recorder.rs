//! Integration tests for the agent-bus in-memory recorder.

use chrono::Utc;
use codetether_agent::bus::recorder::BusRecorder;
use codetether_agent::bus::{BusEnvelope, BusMessage};

fn env(topic: &str) -> BusEnvelope {
    BusEnvelope {
        id: topic.to_string(),
        topic: topic.to_string(),
        sender_id: "t".to_string(),
        correlation_id: None,
        timestamp: Utc::now(),
        message: BusMessage::Heartbeat {
            agent_id: "a".to_string(),
            status: "ok".to_string(),
        },
    }
}

#[test]
fn evicts_oldest_past_capacity() {
    let r = BusRecorder::new(2);
    r.record(&env("swarm.1"));
    r.record(&env("swarm.2"));
    r.record(&env("swarm.3"));
    let recent = r.recent(10, None);
    assert_eq!(recent.len(), 2);
    assert_eq!(recent[0].topic, "swarm.2");
    assert_eq!(recent[1].topic, "swarm.3");
}

#[test]
fn filters_by_topic_prefix_and_limit() {
    let r = BusRecorder::new(10);
    r.record(&env("swarm.a"));
    r.record(&env("task.b"));
    r.record(&env("swarm.c"));
    let swarm = r.recent(1, Some("swarm."));
    assert_eq!(swarm.len(), 1);
    assert_eq!(swarm[0].topic, "swarm.c");
}
