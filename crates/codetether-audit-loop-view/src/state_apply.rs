//! Apply audit-loop progress events to mutable view state.
use crate::{
    audit_loop_event::{AuditLoopEvent, new_iteration},
    audit_loop_reduce as reduce,
    audit_loop_state::AuditLoopState,
};
impl AuditLoopState {
    /// Apply one loop event, mutating the state.
    /// # Arguments
    /// * `event` - Progress event to reduce into the current state.
    /// # Returns
    /// Unit after updating the affected state fields.
    /// # Examples
    /// ```
    /// use codetether_audit_loop_view::{audit_loop_state::AuditLoopState, audit_loop_event::AuditLoopEvent};
    /// let mut state = AuditLoopState::new();
    /// state.apply(AuditLoopEvent::Started { task: "check".into(), max_iterations: 2 });
    /// assert!(state.active);
    /// ```
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
}
