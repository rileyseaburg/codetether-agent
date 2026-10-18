//! Bind model execution and policy-aware verification and commit effects.
use super::*;
#[async_trait::async_trait]
impl codetether_ralph_sequential::Execution for RalphLoop {
    async fn call_llm(&self, story_id: &str, prompt: &str) -> anyhow::Result<String> {
        self.call_llm(story_id, prompt).await
    }
    async fn run_quality_gates_with_events(&self, story_id: &str) -> anyhow::Result<bool> {
        self.run_quality_gates_with_events(story_id).await
    }
    async fn verify(&self, root: &Path, story: &UserStory) -> anyhow::Result<()> {
        run_story_verification(root, story).await
    }
    fn commit_story(&self, story: &UserStory) -> anyhow::Result<()> {
        self.commit_story(story)
    }
}
