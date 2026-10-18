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
#[path = "started_marks_active_and_clears_tests.rs"]
mod started_marks_active_and_clears_tests;

#[path = "gate_started_then_failed_tests.rs"]
mod gate_started_then_failed_tests;

#[path = "iteration_and_complete_status_tests.rs"]
mod iteration_and_complete_status_tests;
