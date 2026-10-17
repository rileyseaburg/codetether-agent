//! Application callbacks for the shared sequential scheduler.
use super::*;
#[async_trait::async_trait]
impl codetether_ralph_core::scheduling::SequentialHost for RalphLoop {
    fn state(&self) -> &RalphState {
        &self.state
    }
    fn begin(&mut self) -> Option<UserStory> {
        self.begin_sequential_iteration()
    }
    fn prompt(&self, story: &UserStory) -> String {
        self.sequential_prompt(story)
    }
    async fn call(&self, story_id: &str, prompt: &str) -> anyhow::Result<String> {
        self.call_llm(story_id, prompt).await
    }
    async fn response(&mut self, story: &UserStory, response: &str) -> anyhow::Result<()> {
        self.handle_sequential_response(story, response).await
    }
    fn error(&mut self, story: &UserStory, error: &anyhow::Error) {
        self.record_sequential_error(story, error);
    }
}
