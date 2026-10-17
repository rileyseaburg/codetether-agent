//! Nonblocking checkpoint persistence.
use super::*;
/// Queue the checkpoint update when persistence is configured.
///
/// # Arguments
/// `store` is optional; `run_id` identifies the run. Remaining inputs supply
/// the state snapshot and payload for this update.
/// # Returns
/// Returns immediately after scheduling; background errors are logged.
/// # Examples
/// ```rust,no_run
/// # fn example(state: &codetether_ralph_core::RalphState, config: &codetether_ralph_core::RalphConfig, story: &codetether_ralph_core::UserStory) {
/// codetether_ralph_core::state_store::updates::checkpoint(None, "run", state);
/// # }
/// ```
pub fn checkpoint(store: Option<&Arc<dyn RalphStateStore>>, run_id: &str, state: &RalphState) {
    if let Some(store) = store {
        let s = store.clone();
        let rid = run_id.to_owned();
        let prd = state.prd.clone();
        let iter = state.current_iteration;
        spawn_update(async move {
            s.update_prd(&rid, &prd).await?;
            s.update_iteration(&rid, iter).await
        });
    }
}
