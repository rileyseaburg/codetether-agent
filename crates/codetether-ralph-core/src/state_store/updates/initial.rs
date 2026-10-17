//! Nonblocking initial persistence.
use super::*;
/// Queue the initial update when persistence is configured.
///
/// # Arguments
/// `store` is optional; `run_id` identifies the run. Remaining inputs supply
/// the state snapshot and payload for this update.
/// # Returns
/// Returns immediately after scheduling; background errors are logged.
/// # Examples
/// ```rust,no_run
/// # fn example(state: &codetether_ralph_core::RalphState, config: &codetether_ralph_core::RalphConfig, story: &codetether_ralph_core::UserStory) {
/// codetether_ralph_core::state_store::updates::initial(None, "run", state, config);
/// # }
/// ```
pub fn initial(
    store: Option<&Arc<dyn RalphStateStore>>,
    run_id: &str,
    state: &RalphState,
    config: &RalphConfig,
) {
    if let Some(store) = store {
        let initial_state = RalphRunState {
            run_id: run_id.to_owned(),
            okr_id: None,
            prd: state.prd.clone(),
            config: config.clone(),
            status: RalphStatus::Running,
            current_iteration: 0,
            max_iterations: state.max_iterations,
            progress_log: Vec::new(),
            story_results: Vec::new(),
            error: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            started_at: Some(chrono::Utc::now().to_rfc3339()),
            completed_at: None,
        };
        let s = store.clone();
        spawn_update(async move { s.create_run(&initial_state).await });
    }
}
