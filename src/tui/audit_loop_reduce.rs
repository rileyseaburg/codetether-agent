//! Reducer helpers that mutate audit-loop iterations in place.

use super::audit_loop_event::new_gate;
use super::audit_loop_model::{GateStatus, IterationStatus};
use super::audit_loop_state::AuditLoopState;

/// Mark a new gate as running on the current iteration.
pub(super) fn gate_started(state: &mut AuditLoopState, name: String) {
    if let Some(it) = state.iterations.last_mut() {
        let mut g = new_gate(name);
        g.status = GateStatus::Running;
        it.gates.push(g);
    }
}

/// Resolve a named gate on the current iteration with a pass/fail result.
pub(super) fn gate_finished(
    state: &mut AuditLoopState,
    name: String,
    passed: bool,
    detail: Option<String>,
) {
    if let Some(it) = state.iterations.last_mut()
        && let Some(g) = it.gates.iter_mut().find(|g| g.name == name)
    {
        g.status = if passed {
            GateStatus::Passed
        } else {
            GateStatus::Failed
        };
        g.detail = detail;
    }
}

/// Set the terminal status of the current iteration.
pub(super) fn iteration_finished(state: &mut AuditLoopState, passed: bool) {
    if let Some(it) = state.iterations.last_mut() {
        it.status = if passed {
            IterationStatus::Passed
        } else {
            IterationStatus::Failed
        };
    }
}
