//! Select relay execution or the existing single-agent fallback.
use super::parallel_execution_context::ParallelExecutionContext;
use super::*;
impl ParallelExecutionContext<'_> {
    pub(super) async fn call(&self, prompt: &str) -> anyhow::Result<String> {
        if self.relay_enabled {
            if let Some(reg) = self.registry {
                RalphLoop::call_relay_static(
                    reg,
                    self.model,
                    prompt,
                    self.working_dir,
                    self.ralph_tx.clone(),
                    self.story.id.clone(),
                    self.bus.clone(),
                    self.relay_max_agents,
                    self.relay_max_rounds,
                    self.delegation,
                )
                .await
            } else {
                warn!(story_id = %self.story.id, "Relay enabled but no registry available, using single agent");
                self.call_direct(prompt).await
            }
        } else {
            self.call_direct(prompt).await
        }
    }
}
