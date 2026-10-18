//! Persist and announce the terminal run state.
use super::*;
pub(crate) fn finish_run<H: Host>(host: &H) -> anyhow::Result<RalphState> {
    // Persist final state to store
    codetether_ralph_core::state_store::updates::completion(
        host.store(),
        host.run_id(),
        host.state(),
    );

    info!(target: "codetether_agent::ralph::ralph_loop::lifecycle_finish",
        "Ralph finished: {:?}, {}/{} stories passed",
        host.state().status,
        host.state().prd.passed_count(),
        host.state().prd.user_stories.len()
    );

    // Emit Complete event
    host.try_send_event(RalphEvent::Complete {
        status: format!("{:?}", host.state().status),
        passed: host.state().prd.passed_count(),
        total: host.state().prd.user_stories.len(),
    });

    Ok(host.state().clone())
}
