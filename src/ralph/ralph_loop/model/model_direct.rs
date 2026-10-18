//! Bind instance stories to shared Ralph agent execution.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) async fn call_llm(
        &self,
        story_id: &str,
        prompt: &str,
    ) -> anyhow::Result<String> {
        codetether_ralph_agent::direct::<super::host::Native>(
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
