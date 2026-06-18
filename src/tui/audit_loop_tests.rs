//! Tests for the audit-loop state reducer.

use super::audit_loop_event::AuditLoopEvent;
use super::audit_loop_model::{GateStatus, IterationStatus};
use super::audit_loop_state::AuditLoopState;

#[path = "audit_loop_lines_tests.rs"]
mod lines_tests;

/// Build a state mid-loop: started, one iteration, one running gate.
pub(super) fn run() -> AuditLoopState {
    let mut s = AuditLoopState::new();
    s.apply(AuditLoopEvent::Started {
        task: "tables".into(),
        max_iterations: 5,
    });
    s.apply(AuditLoopEvent::IterationStarted {
        number: 1,
        summary: "first attempt".into(),
    });
    s.apply(AuditLoopEvent::GateStarted {
        name: "test".into(),
    });
    s
}

#[test]
fn started_marks_active_and_clears() {
    let s = run();
    assert!(s.active);
    assert_eq!(s.task, "tables");
    assert_eq!(s.iterations.len(), 1);
}

#[test]
fn gate_started_then_failed() {
    let mut s = run();
    s.apply(AuditLoopEvent::GateFinished {
        name: "test".into(),
        passed: false,
        detail: Some("2 failed".into()),
    });
    let g = &s.iterations[0].gates[0];
    assert_eq!(g.status, GateStatus::Failed);
    assert_eq!(g.detail.as_deref(), Some("2 failed"));
}

#[test]
fn iteration_and_complete_status() {
    let mut s = run();
    s.apply(AuditLoopEvent::IterationFinished { passed: true });
    assert_eq!(s.iterations[0].status, IterationStatus::Passed);
    s.apply(AuditLoopEvent::Complete {
        passed: true,
        iterations: 1,
    });
    assert!(s.complete && s.passed);
}
