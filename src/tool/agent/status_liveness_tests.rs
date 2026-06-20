//! Tests for sub-agent liveness classification.

use super::status_liveness::{liveness, STALL_SECS};
use crate::a2a::types::TaskState;
use chrono::{Duration, Utc};

#[test]
fn terminal_states_are_settled() {
    let now = Utc::now();
    assert_eq!(liveness(&TaskState::Completed, now), "settled");
    assert_eq!(liveness(&TaskState::Failed, now), "settled");
}

#[test]
fn fresh_working_is_active() {
    assert_eq!(liveness(&TaskState::Working, Utc::now()), "active");
}

#[test]
fn silent_working_is_stalled() {
    let stale = Utc::now() - Duration::seconds(STALL_SECS + 5);
    assert_eq!(liveness(&TaskState::Working, stale), "stalled");
}

#[test]
fn submitted_is_queued() {
    assert_eq!(liveness(&TaskState::Submitted, Utc::now()), "queued");
}

#[test]
fn input_required_is_waiting() {
    assert_eq!(liveness(&TaskState::InputRequired, Utc::now()), "waiting");
}
