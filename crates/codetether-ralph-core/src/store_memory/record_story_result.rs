//! record_story_result persistence operation.
use super::*;
impl InMemoryStore {
    pub(super) async fn record_story_result_impl(
        &self,
        run_id: &str,
        result: &StoryResultEntry,
    ) -> anyhow::Result<()> {
        let mut runs = self.runs.write().await;
        if let Some(run) = runs.get_mut(run_id) {
            // Update or insert
            if let Some(existing) = run
                .story_results
                .iter_mut()
                .find(|r| r.story_id == result.story_id)
            {
                *existing = result.clone();
            } else {
                run.story_results.push(result.clone());
            }
        }
        Ok(())
    }
}
