//! Nonblocking story persistence.
use super::*;
/// Queue the story update when persistence is configured.
///
/// # Arguments
/// `store` is optional; `run_id` identifies the run. Remaining inputs supply
/// the state snapshot and payload for this update.
/// # Returns
/// Returns immediately after scheduling; background errors are logged.
/// # Examples
/// ```rust,no_run
/// # fn example(state: &codetether_ralph_core::RalphState, config: &codetether_ralph_core::RalphConfig, story: &codetether_ralph_core::UserStory) {
/// codetether_ralph_core::state_store::updates::story(None, "run", state, story, false, None);
/// # }
/// ```
pub fn story(
    store: Option<&Arc<dyn RalphStateStore>>,
    run_id: &str,
    state: &RalphState,
    story: &UserStory,
    passed: bool,
    error: Option<String>,
) {
    if let Some(store) = store {
        let s = store.clone();
        let rid = run_id.to_owned();
        let entry = StoryResultEntry {
            story_id: story.id.clone(),
            title: story.title.clone(),
            passed,
            iteration: state.current_iteration,
            error,
        };
        spawn_update(async move { s.record_story_result(&rid, &entry).await });
    }
}
