//! Sequential scheduling loop with focused per-iteration handlers.
use super::*;
impl RalphLoop {
    pub(super) async fn run_sequential(&mut self) -> anyhow::Result<()> {
        while self.state.current_iteration < self.state.max_iterations {
            let Some(story) = self.begin_sequential_iteration() else {
                break;
            };
            let prompt = self.sequential_prompt(&story);
            match self.call_llm(&story.id, &prompt).await {
                Ok(response) => self.handle_sequential_response(&story, &response).await?,
                Err(e) => self.record_sequential_error(&story, &e),
            }
        }
        Ok(())
    }
}
