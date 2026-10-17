//! record_story_result persistence operation.
use super::*;
impl HttpStore {
    pub(super) async fn record_story_result_impl(
        &self,
        run_id: &str,
        result: &StoryResultEntry,
    ) -> anyhow::Result<()> {
        let url = format!(
            "{}/v1/ralph/runs/{}/stories/{}",
            self.base_url, run_id, result.story_id
        );
        let body = json!({
            "story_id": result.story_id,
            "title": result.title,
            "passed": result.passed,
            "iteration": result.iteration,
            "error": result.error,
        });

        let req = self.apply_auth(self.client.put(&url)).json(&body);
        if let Err(e) = req.send().await {
            warn!(run_id = %run_id, story = %result.story_id, error = %e, "HTTP store record_story_result error");
        }
        Ok(())
    }
}
