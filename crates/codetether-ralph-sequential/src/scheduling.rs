//! Application callbacks for the shared sequential scheduler.
use crate::*;
#[async_trait::async_trait]
impl<H: Host> codetether_ralph_core::scheduling::SequentialHost for Runtime<'_, H> {
    fn state(&self) -> &RalphState {
        self.host.state()
    }
    fn begin(&mut self) -> Option<UserStory> {
        begin_sequential_iteration(self.host)
    }
    fn prompt(&self, story: &UserStory) -> String {
        sequential_prompt(self.host, story)
    }
    async fn call(&self, story_id: &str, prompt: &str) -> anyhow::Result<String> {
        self.host.call_llm(story_id, prompt).await
    }
    async fn response(&mut self, story: &UserStory, response: &str) -> anyhow::Result<()> {
        handle_sequential_response(self.host, story, response).await
    }
    fn error(&mut self, story: &UserStory, error: &anyhow::Error) {
        record_sequential_error(self.host, story, error);
    }
}
