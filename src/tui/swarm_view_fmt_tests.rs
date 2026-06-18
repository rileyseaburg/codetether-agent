//! Tests for swarm subtask row formatting.

use super::super::SubTaskInfo;
use super::{elapsed_label, status_glyph};
use crate::swarm::SubTaskStatus;

fn task(status: SubTaskStatus) -> SubTaskInfo {
    SubTaskInfo {
        id: "t1".into(),
        name: "demo".into(),
        status,
        stage: 0,
        dependencies: vec![],
        agent_name: None,
        current_tool: None,
        steps: 0,
        max_steps: 20,
        tool_call_history: vec![],
        messages: vec![],
        output: None,
        error: None,
        started_at: None,
        elapsed_secs: None,
    }
}

#[test]
fn elapsed_uses_frozen_value_when_set() {
    let mut t = task(SubTaskStatus::Completed);
    t.elapsed_secs = Some(75);
    assert_eq!(elapsed_label(&t).as_deref(), Some("1m15s"));
}

#[test]
fn elapsed_formats_seconds_under_a_minute() {
    let mut t = task(SubTaskStatus::Completed);
    t.elapsed_secs = Some(42);
    assert_eq!(elapsed_label(&t).as_deref(), Some("42s"));
}

#[test]
fn elapsed_is_none_for_pending_without_start() {
    let t = task(SubTaskStatus::Pending);
    assert!(elapsed_label(&t).is_none());
}

#[test]
fn running_glyph_is_cyan_dot() {
    let (icon, _) = status_glyph(SubTaskStatus::Running);
    assert_eq!(icon, "●");
}
