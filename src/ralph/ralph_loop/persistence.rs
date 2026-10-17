//! Publish story results through nonblocking persistence tasks.
use super::*;
impl RalphLoop {
    /// Fire-and-forget store update — logs errors but never blocks
    pub(super) fn store_fire_and_forget(
        &self,
        fut: impl std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
    ) {
        tokio::spawn(async move {
            if let Err(e) = fut.await {
                warn!(error = %e, "State store update failed");
            }
        });
    }

    pub(super) fn record_story_result(
        &self,
        story: &UserStory,
        passed: bool,
        error: Option<String>,
    ) {
        if let Some(ref store) = self.store {
            let s = store.clone();
            let rid = self.run_id.clone();
            let entry = StoryResultEntry {
                story_id: story.id.clone(),
                title: story.title.clone(),
                passed,
                iteration: self.state.current_iteration,
                error,
            };
            self.store_fire_and_forget(async move { s.record_story_result(&rid, &entry).await });
        }
    }
}
