//! Native session identity, cancellation and continuation callbacks.
use super::Native;
impl codetether_goal_answer::InputActions for Native<'_> {
    type Journal = super::super::review_host::Native;
    fn session_id(&self) -> String {
        self.slot.view().id().to_string()
    }
    fn cancel(&mut self) {
        self.runtime.request_cancel_current();
    }
    fn restart(&mut self, id: &str) {
        super::super::commands::restart(self.app, id, self.runtime);
    }
}
