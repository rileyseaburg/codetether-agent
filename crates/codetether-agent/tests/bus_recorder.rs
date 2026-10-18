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

#[test]
fn drain_since_delivers_each_envelope_once() {
    let r = BusRecorder::new(100);
    let start = r.cursor();
    r.record(&env("swarm.1"));
    r.record(&env("swarm.2"));
    let (batch, c1) = r.drain_since(start);
    assert_eq!(batch.len(), 2);
    assert_eq!(batch[0].topic, "swarm.1");
    assert_eq!(batch[1].topic, "swarm.2");
    // Nothing new since c1.
    let (empty, c2) = r.drain_since(c1);
    assert!(empty.is_empty());
    assert_eq!(c1, c2);
    // New record shows up exactly once.
    r.record(&env("swarm.3"));
    let (batch3, _) = r.drain_since(c2);
    assert_eq!(batch3.len(), 1);
    assert_eq!(batch3[0].topic, "swarm.3");
}

#[test]
fn drain_since_survives_high_volume_without_skipping_window() {
    // The bug: a lossy live receiver skips on lag. The cursor drain must
    // still return everything within the retained window in order.
    let r = BusRecorder::new(1000);
    let start = r.cursor();
    for i in 0..500 {
        r.record(&env(&format!("tools.{i}")));
    }
    let (batch, _) = r.drain_since(start);
    assert_eq!(batch.len(), 500);
    assert_eq!(batch[0].topic, "tools.0");
    assert_eq!(batch[499].topic, "tools.499");
}
