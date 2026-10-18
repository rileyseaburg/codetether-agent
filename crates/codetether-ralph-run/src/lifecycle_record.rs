//! Create the initial persistent run record without blocking execution.
use super::*;
pub(crate) fn create_initial_run_record<H: Host>(host: &H) {
    // Create run record in store
    codetether_ralph_core::state_store::updates::initial(
        host.store(),
        host.run_id(),
        host.state(),
        host.config(),
    );
}
