//! Tests for tool-activity backfill of sub-agent status (issue #295).

use super::merge;
use crate::a2a::types::TaskState;
use crate::tool::agent::status_source::AgentStatus;
use chrono::{Duration, Utc};
use std::collections::HashMap;

#[test]
fn activity_backfills_agent_without_task_update() {
    let mut out = HashMap::new();
    let now = Utc::now();
    merge(&mut out, HashMap::from([("worker".to_string(), now)]));
    assert_eq!(out["worker"].state, TaskState::Working);
}

#[test]
fn newer_task_update_is_not_overwritten_by_older_activity() {
    let now = Utc::now();
    let mut out = HashMap::from([(
        "worker".to_string(),
        AgentStatus {
            state: TaskState::Completed,
            summary: None,
            at: now,
        },
    )]);
    let older = now - Duration::seconds(30);
    merge(&mut out, HashMap::from([("worker".to_string(), older)]));
    assert_eq!(out["worker"].state, TaskState::Completed);
}
