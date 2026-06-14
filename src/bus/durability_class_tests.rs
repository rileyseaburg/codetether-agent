//! Unit tests for the durable/presence classifier.

use crate::a2a::types::TaskState;
use crate::bus::BusMessage;

#[test]
fn coordination_is_durable() {
    let m = BusMessage::TaskUpdate {
        task_id: "t1".into(),
        state: TaskState::Working,
        message: None,
    };
    assert!(m.is_durable());
    assert_eq!(m.partition_key(), Some("t1"));
}

#[test]
fn presence_is_not_durable() {
    let m = BusMessage::Heartbeat {
        agent_id: "a1".into(),
        status: "ok".into(),
    };
    assert!(!m.is_durable());
    assert_eq!(m.partition_key(), None);
}

#[test]
fn ralph_handoff_partitions_by_prd() {
    let m = BusMessage::RalphProgress {
        prd_id: "p9".into(),
        passed: 1,
        total: 3,
        iteration: 2,
        status: "running".into(),
    };
    assert!(m.is_durable());
    assert_eq!(m.partition_key(), Some("p9"));
}
