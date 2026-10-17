//! Nonblocking completion persistence.
use super::*;
/// Queue the completion update when persistence is configured.
///
/// # Arguments
/// `store` is optional; `run_id` identifies the run. Remaining inputs supply
/// the state snapshot and payload for this update.
/// # Returns
/// Returns immediately after scheduling; background errors are logged.
/// # Examples
/// ```rust,no_run
/// # fn example(state: &codetether_ralph_core::RalphState, config: &codetether_ralph_core::RalphConfig, story: &codetether_ralph_core::UserStory) {
/// codetether_ralph_core::state_store::updates::completion(None, "run", state);
/// # }
/// ```
pub fn completion(store: Option<&Arc<dyn RalphStateStore>>, run_id: &str, state: &RalphState) {
    if let Some(store) = store {
        let s = store.clone();
        let rid = run_id.to_owned();
        let status = state.status;
        let prd = state.prd.clone();
        spawn_update(async move {
            s.update_prd(&rid, &prd).await?;
            s.complete_run(&rid, status).await
        });
    }
}
