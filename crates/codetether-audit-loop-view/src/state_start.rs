//! Reset view state at the start of a new audit loop.
use crate::audit_loop_state::AuditLoopState;
impl AuditLoopState {
    pub(crate) fn start(&mut self, task: String, max_iterations: usize) {
        self.active = true;
        self.task = task;
        self.max_iterations = max_iterations;
        self.iterations.clear();
        self.complete = false;
    }
}
