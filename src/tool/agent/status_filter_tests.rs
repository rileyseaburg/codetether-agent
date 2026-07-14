//! Regression tests for parent-scoped lifecycle status.

use super::collect;
use crate::a2a::types::TaskState;
use crate::bus::{BusEnvelope, BusMessage};
use chrono::Utc;
use std::collections::HashMap;

fn update(task_id: &str) -> BusEnvelope {
    BusEnvelope {
        id: task_id.into(),
        topic: format!("task.{task_id}"),
        sender_id: "child".into(),
        correlation_id: None,
        timestamp: Utc::now(),
        message: BusMessage::TaskUpdate {
            task_id: task_id.into(),
            state: TaskState::Failed,
            message: Some("startup error".into()),
        },
    }
}

#[test]
fn unrelated_global_task_is_excluded() {
    let owners = HashMap::from([("agent-session:owned".into(), "luna".into())]);
    let states = collect(
        vec![
            update("agent-session:foreign"),
            update("agent-session:owned"),
        ],
        &owners,
    );
    assert_eq!(states.len(), 1);
    assert_eq!(states["luna"].summary.as_deref(), Some("startup error"));
}
