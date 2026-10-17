//! Create the initial persistent run record without blocking execution.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn create_initial_run_record(&self) {
        // Create run record in store
        codetether_ralph_core::state_store::updates::initial(
            self.store.as_ref(),
            &self.run_id,
            &self.state,
            &self.config,
        );
    }
}
