//! Native session field binding for shared run configuration policy.
pub(super) use codetether_run_policy::{apply_session_policy, effective_access_mode, load};
impl codetether_run_policy::SessionPolicy for crate::session::Session {
    fn set_model(&mut self, model: String) {
        self.metadata.model = Some(model);
    }
    fn enable_auto_apply(&mut self) {
        self.metadata.auto_apply_edits = true;
    }
    fn set_max_steps(&mut self, max_steps: Option<usize>) {
        self.max_steps = max_steps;
    }
}
