//! Bind instance stories to shared Ralph agent execution.
use super::*;
impl<H: Host> RalphLoop<H> {
    pub(crate) async fn call_llm(&self, story_id: &str, prompt: &str) -> anyhow::Result<String> {
        codetether_ralph_agent::direct::<H::Agent>(
            &self.provider,
            &self.model,
            &self.state.working_dir,
            story_id,
            prompt,
            self.event_tx.as_ref(),
            &self.bus,
        )
        .await
    }
}
