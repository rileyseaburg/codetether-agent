//! View state and event reducer for the audit-loop view.

use super::audit_loop_event::{AuditLoopEvent, new_iteration};
use super::audit_loop_model::Iteration;
use super::audit_loop_reduce as reduce;

/// Mutable state backing the audit-loop view.
#[derive(Debug, Default)]
pub struct AuditLoopState {
    /// Whether the view is active/has data.
    pub active: bool,
    /// Task label being worked.
    pub task: String,
    /// Retry cap.
    pub max_iterations: usize,
    /// All iterations, oldest first.
    pub iterations: Vec<Iteration>,
    /// Whether the loop has finished.
    pub complete: bool,
    /// Final pass/fail once complete.
    pub passed: bool,
    /// Selected iteration index for navigation.
    pub selected: usize,
}

impl AuditLoopState {
    /// Create an empty state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Apply one loop event, mutating the state.
    pub fn apply(&mut self, event: AuditLoopEvent) {
        match event {
            AuditLoopEvent::Started {
                task,
                max_iterations,
            } => self.start(task, max_iterations),
            AuditLoopEvent::IterationStarted { number, summary } => {
                self.iterations.push(new_iteration(number, summary));
                self.selected = self.iterations.len().saturating_sub(1);
            }
            AuditLoopEvent::GateStarted { name } => reduce::gate_started(self, name),
            AuditLoopEvent::GateFinished {
                name,
                passed,
                detail,
            } => reduce::gate_finished(self, name, passed, detail),
            AuditLoopEvent::IterationFinished { passed } => {
                reduce::iteration_finished(self, passed)
            }
            AuditLoopEvent::Complete { passed, .. } => {
                self.complete = true;
                self.passed = passed;
            }
        }
    }

    fn start(&mut self, task: String, max_iterations: usize) {
        self.active = true;
        self.task = task;
        self.max_iterations = max_iterations;
        self.iterations.clear();
        self.complete = false;
    }
}
