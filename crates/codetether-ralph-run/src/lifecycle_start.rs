//! Start the run, announce it, and select its feature branch.
use super::*;
pub(crate) fn start_run<H: Host>(host: &mut H) -> anyhow::Result<()> {
    host.state_mut().status = RalphStatus::Running;
    codetether_ralph_core::verifiability::warn_unverifiable_stories(
        &host.state().prd,
        host.config().quality_checks_enabled,
    );
    create_initial_run_record(host);
    // Emit Started event
    host.try_send_event(RalphEvent::Started {
        project: host.state().prd.project.clone(),
        feature: host.state().prd.feature.clone(),
        stories: codetether_ralph_core::events::initial_stories(&host.state().prd),
        max_iterations: host.state().max_iterations,
    });
    // Switch to feature branch
    if !host.state().prd.branch_name.is_empty() {
        info!(target: "codetether_agent::ralph::ralph_loop::lifecycle_start", branch = %host.state().prd.branch_name, "Switching to branch");
        host.git_checkout(&host.state().prd.branch_name)?;
    }

    Ok(())
}
