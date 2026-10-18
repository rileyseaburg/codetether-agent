//! Top-level Ralph run lifecycle and scheduling-mode selection.
use super::*;

/// Run Ralph through startup, selected scheduling and terminal reporting.
/// # Arguments
/// * `host` - Live state and native execution services.
/// # Returns
/// A clone of the final run state after cleanup and completion notification.
/// # Errors
/// Propagates startup and scheduler errors before terminal cleanup/reporting.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &mut impl codetether_ralph_run::Host) -> anyhow::Result<()> {
/// let final_state = codetether_ralph_run::run(host).await?;
/// assert_ne!(final_state.status, codetether_ralph_core::RalphStatus::Running);
/// # Ok(()) }
/// ```
pub async fn run<H: Host>(host: &mut H) -> anyhow::Result<RalphState> {
    start_run(host)?;
    // Choose execution mode
    if host.config().parallel_enabled {
        host.run_parallel().await?;
    } else {
        host.run_sequential().await?;
    }

    codetether_ralph_core::lifecycle::finalize_status(host.state_mut());
    cleanup_run_worktrees(host).await;
    finish_run(host)
}
