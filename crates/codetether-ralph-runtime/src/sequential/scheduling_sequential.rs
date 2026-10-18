//! Application callbacks for the shared sequential scheduler.
use super::*;
#[async_trait::async_trait]
impl<H: Host> codetether_ralph_core::scheduling::SequentialHost for RalphLoop<H> {
    fn state(&self) -> &RalphState {
        &self.state
    }
    fn begin(&mut self) -> Option<UserStory> {
        codetether_ralph_sequential::begin_sequential_iteration(self)
    }
    fn prompt(&self, story: &UserStory) -> String {
        codetether_ralph_sequential::sequential_prompt(self, story)
    }
    async fn call(&self, story_id: &str, prompt: &str) -> anyhow::Result<String> {
        self.call_llm(story_id, prompt).await
    }
    async fn response(&mut self, story: &UserStory, response: &str) -> anyhow::Result<()> {
        codetether_ralph_sequential::handle_sequential_response(self, story, response).await
    }
    fn error(&mut self, story: &UserStory, error: &anyhow::Error) {
        codetether_ralph_sequential::record_sequential_error(self, story, error);
    }
}
