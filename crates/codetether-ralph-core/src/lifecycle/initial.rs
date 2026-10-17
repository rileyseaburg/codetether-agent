//! Load the PRD and initialize resumable execution state.
use crate::{Prd, RalphState, RalphStatus};
use std::path::PathBuf;
/// Load a pending run, resuming its iteration count from passed stories.
///
/// # Arguments
/// `prd_path` selects the PRD; `max_iterations` sets its execution limit.
/// `on_load` observes loaded metadata after working-directory resolution.
/// # Returns
/// Pending state with an empty progress log and the original PRD path.
/// # Errors
/// Returns PRD loading/parsing or current-directory resolution errors.
/// # Examples
/// ```rust,no_run
/// async fn example() -> anyhow::Result<()> {
/// let state = codetether_ralph_core::lifecycle::load_initial_state(
///     &"prd.json".into(), 10, |_| {}).await?;
/// assert_eq!(state.status, codetether_ralph_core::RalphStatus::Pending);
/// Ok(())
/// }
/// ```
pub async fn load_initial_state(
    prd_path: &PathBuf,
    max_iterations: usize,
    on_load: impl FnOnce(&Prd),
) -> anyhow::Result<RalphState> {
    let prd = Prd::load(prd_path).await?;

    // Get working directory - use parent of prd_path or current directory
    let working_dir = if let Some(parent) = prd_path.parent() {
        if parent.as_os_str().is_empty() {
            std::env::current_dir()?
        } else {
            parent.to_path_buf()
        }
    } else {
        std::env::current_dir()?
    };

    on_load(&prd);

    // Resume iteration counter: each passed story consumed one iteration
    let resumed_iterations = prd.passed_count();

    let state = RalphState {
        prd,
        current_iteration: resumed_iterations,
        max_iterations,
        status: RalphStatus::Pending,
        progress_log: Vec::new(),
        prd_path: prd_path.clone(),
        working_dir,
    };

    Ok(state)
}
