//! Select relay execution or the existing single-agent fallback.
use super::parallel_execution_context::ParallelExecutionContext;
use super::*;
impl<H: Host> ParallelExecutionContext<'_, H> {
    pub(crate) async fn call(&self, prompt: &str) -> anyhow::Result<String> {
        if self.relay_enabled {
            if let Some(reg) = self.registry {
                H::relay(self, reg, prompt).await
            } else {
                warn!(target: "codetether_agent::ralph::ralph_loop::parallel_call_mode", story_id = %self.story.id, "Relay enabled but no registry available, using single agent");
                self.call_direct(prompt).await
            }
        } else {
            self.call_direct(prompt).await
        }
    }
}
