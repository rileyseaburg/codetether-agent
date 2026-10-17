//! Persist and announce the terminal run state.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn finish_run(&self) -> anyhow::Result<RalphState> {
        // Persist final state to store
        codetether_ralph_core::state_store::updates::completion(
            self.store.as_ref(),
            &self.run_id,
            &self.state,
        );

        info!(target: "codetether_agent::ralph::ralph_loop::lifecycle_finish",
            "Ralph finished: {:?}, {}/{} stories passed",
            self.state.status,
            self.state.prd.passed_count(),
            self.state.prd.user_stories.len()
        );

        // Emit Complete event
        self.try_send_event(RalphEvent::Complete {
            status: format!("{:?}", self.state.status),
            passed: self.state.prd.passed_count(),
            total: self.state.prd.user_stories.len(),
        });

        Ok(self.state.clone())
    }
}
